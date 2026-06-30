//! Integration tests for IA pipeline using a mock gRPC server.

use futures::StreamExt;
use std::path::PathBuf;
use strata_ia_bridge::proto::{
    ia_service_server::{IaService, IaServiceServer},
    Crop, FormulaResponse, ImageDescription, ImageResponse, OcrResponse, OcrResult, Provenance,
    StreamCrop, StreamResult, TableResponse,
};
use strata_pipeline::{parse_document, ParsePipelineOptions};
use tokio::sync::oneshot;
use tonic::{transport::Server, Request, Response, Status, Streaming};

#[derive(Default)]
struct MockIa;

#[tonic::async_trait]
impl IaService for MockIa {
    async fn ocr_page(&self, _req: Request<Crop>) -> Result<Response<OcrResponse>, Status> {
        Ok(Response::new(OcrResponse {
            result: Some(OcrResult {
                text: "Mocked OCR Text".into(),
                words: vec![],
                confidence: 0.95,
                language: "eng".into(),
            }),
            provenance: Some(canned_provenance()),
        }))
    }

    async fn extract_table(&self, _req: Request<Crop>) -> Result<Response<TableResponse>, Status> {
        Ok(Response::new(TableResponse {
            result: Some(strata_ia_bridge::proto::TableResult {
                rows: vec![],
                confidence: 0.9,
                cell_count: 0,
            }),
            provenance: Some(canned_provenance()),
        }))
    }

    async fn describe_image(&self, _req: Request<Crop>) -> Result<Response<ImageResponse>, Status> {
        Ok(Response::new(ImageResponse {
            result: Some(ImageDescription {
                caption: "Mocked Caption".into(),
                description: "Mocked Description".into(),
                alt_text: "Mocked Alt Text".into(),
                confidence: 0.85,
            }),
            provenance: Some(canned_provenance()),
        }))
    }

    async fn ocr_formula(&self, _req: Request<Crop>) -> Result<Response<FormulaResponse>, Status> {
        Ok(Response::new(FormulaResponse {
            result: Some(strata_ia_bridge::proto::FormulaResult {
                latex: "E=mc^2".into(),
                mathml: "".into(),
                confidence: 0.99,
            }),
            provenance: Some(canned_provenance()),
        }))
    }

    type ProcessStreamStream =
        std::pin::Pin<Box<dyn futures::Stream<Item = Result<StreamResult, Status>> + Send>>;

    async fn process_stream(
        &self,
        req: Request<Streaming<StreamCrop>>,
    ) -> Result<Response<Self::ProcessStreamStream>, Status> {
        let mut inbound = req.into_inner();
        let stream = async_stream::try_stream! {
            while let Some(crop) = inbound.next().await {
                let crop = crop?;
                let payload = match crop.route {
                    r if r == strata_ia_bridge::proto::TriageRoute::Table as i32 => {
                        strata_ia_bridge::proto::stream_result::Payload::Table(strata_ia_bridge::proto::TableResponse {
                            result: Some(strata_ia_bridge::proto::TableResult {
                                rows: vec![],
                                confidence: 0.9,
                                cell_count: 0,
                            }),
                            provenance: Some(canned_provenance()),
                        })
                    }
                    r if r == strata_ia_bridge::proto::TriageRoute::Image as i32 => {
                        strata_ia_bridge::proto::stream_result::Payload::Image(strata_ia_bridge::proto::ImageResponse {
                            result: Some(ImageDescription {
                                caption: "Mocked Caption".into(),
                                description: "Mocked Description".into(),
                                alt_text: "Mocked Alt Text".into(),
                                confidence: 0.85,
                            }),
                            provenance: Some(canned_provenance()),
                        })
                    }
                    r if r == strata_ia_bridge::proto::TriageRoute::Formula as i32 => {
                        strata_ia_bridge::proto::stream_result::Payload::Formula(strata_ia_bridge::proto::FormulaResponse {
                            result: Some(strata_ia_bridge::proto::FormulaResult {
                                latex: "E=mc^2".into(),
                                mathml: "".into(),
                                confidence: 0.99,
                            }),
                            provenance: Some(canned_provenance()),
                        })
                    }
                    _ => {
                        strata_ia_bridge::proto::stream_result::Payload::Ocr(OcrResponse {
                            result: Some(OcrResult {
                                text: "Mocked OCR Text".into(),
                                words: vec![],
                                confidence: 0.95,
                                language: "eng".into(),
                            }),
                            provenance: Some(canned_provenance()),
                        })
                    }
                };
                yield StreamResult {
                    correlation_id: crop.correlation_id,
                    payload: Some(payload),
                };
            }
        };
        Ok(Response::new(Box::pin(stream)))
    }
}

fn canned_provenance() -> Provenance {
    Provenance {
        model_id: "mock".into(),
        backend: "mock".into(),
        latency_ms: 10,
        retries: 0,
        cache_hit: false,
    }
}

async fn spawn_server() -> (tokio::task::JoinHandle<()>, String, oneshot::Sender<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (tx, rx) = oneshot::channel::<()>();
    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(IaServiceServer::new(MockIa))
            .serve_with_incoming_shutdown(
                tokio_stream::wrappers::TcpListenerStream::new(listener),
                async move {
                    let _ = rx.await;
                },
            )
            .await
            .ok();
    });
    (handle, format!("http://{addr}"), tx)
}

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("tests/fixtures/pdfs/crecks")
        .join(name)
}

#[tokio::test]
async fn test_pipeline_with_mock_grpc_ia_backend() {
    let path = fixture("figure_with_caption.pdf");
    if !path.exists() {
        return;
    }

    let (_server_handle, addr, shutdown_tx) = spawn_server().await;

    let opts = ParsePipelineOptions {
        input: path,
        profile: "scientific".into(),
        use_ia: true,
        force_ocr: false,
        ollama_endpoint: "".into(),
        ia_grpc_endpoint: Some(addr),
        max_concurrent_pages: None,
        media_dir: None,
        save_images: false,
        pdf_backend: "auto".into(),
    };

    let artifacts = match parse_document(opts).await {
        Ok(art) => art,
        Err(strata_pipeline::PipelineError::PdfOpen(msg))
            if msg.contains("pdfium native library") =>
        {
            let _ = shutdown_tx.send(());
            println!("Saltando test: pdfium no disponible");
            return;
        }
        Err(e) => {
            let _ = shutdown_tx.send(());
            panic!("pipeline falló: {:?}", e);
        }
    };

    assert!(!artifacts.document.pages.is_empty(), "debe tener páginas");

    // Clean up
    let _ = shutdown_tx.send(());
}
