//! Background job worker for strata-server.
//!
//! Periodically polls the JobStore for queued jobs, runs the parsing pipeline,
//! and updates status to Done/Failed, saving the final markdown and json artifacts.

use crate::jobs::{JobId, JobStatus, JobStore};
use std::sync::Arc;
use std::time::Duration;
use strata_pipeline::{parse_document, ParsePipelineOptions};
use tracing::{error, info};

pub struct BackgroundWorker {
    store: Arc<dyn JobStore>,
    ollama_endpoint: String,
    use_ia: bool,
}

impl BackgroundWorker {
    pub fn new(store: Arc<dyn JobStore>, ollama_endpoint: String, use_ia: bool) -> Self {
        Self {
            store,
            ollama_endpoint,
            use_ia,
        }
    }

    pub fn start(self) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(500));
            loop {
                interval.tick().await;
                if let Err(e) = self.poll_and_process().await {
                    error!("worker error in loop: {e}");
                }
            }
        });
    }

    async fn poll_and_process(&self) -> Result<(), String> {
        let jobs = self.store.list().await.map_err(|e| e.to_string())?;
        let queued_job = jobs
            .into_iter()
            .find(|j| matches!(j.status, JobStatus::Queued));
        if let Some(mut job) = queued_job {
            info!(job = %job.id, "worker picked up queued job");
            job.status = JobStatus::Running { progress: 0 };
            job.updated_at = unix_seconds();
            self.store
                .put(job.clone())
                .await
                .map_err(|e| e.to_string())?;

            let id = job.id;
            let upload_path = crate::routes::get_upload_path(id);
            let ollama = self.ollama_endpoint.clone();
            let use_ia = self.use_ia;
            let store = self.store.clone();

            tokio::spawn(async move {
                let res = Self::process_job(id, &upload_path, &ollama, use_ia).await;
                if let Ok(Some(mut current_job)) = store.get(id).await {
                    match res {
                        Ok((md, json)) => {
                            current_job.status = JobStatus::Done;
                            current_job.result_md = Some(md);
                            current_job.result_json = Some(json);
                        }
                        Err(e) => {
                            current_job.status = JobStatus::Failed { error: e };
                        }
                    }
                    current_job.updated_at = unix_seconds();
                    let _ = store.put(current_job).await;
                }
                let _ = std::fs::remove_file(&upload_path);
            });
        }
        Ok(())
    }

    async fn process_job(
        _id: JobId,
        path: &std::path::Path,
        ollama: &str,
        use_ia: bool,
    ) -> Result<(String, String), String> {
        let opts = ParsePipelineOptions {
            input: path.to_path_buf(),
            profile: "balanced".into(),
            use_ia,
            force_ocr: false,
            ollama_endpoint: ollama.to_string(),
            ia_grpc_endpoint: None,
            max_concurrent_pages: None,
            media_dir: None,
            save_images: false,
            pdf_backend: "auto".into(),
        };

        let artifacts = parse_document(opts).await.map_err(|e| e.to_string())?;

        let md = strata_serialize::render_markdown(
            &artifacts.document,
            &strata_serialize::MarkdownOptions::default(),
        );
        let graph = strata_serialize::render_graph(&artifacts.document);
        let json = serde_json::to_string_pretty(&graph).map_err(|e| e.to_string())?;

        Ok((md, json))
    }
}

fn unix_seconds() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
