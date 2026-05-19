//! Throughput bench — bidirectional streaming vs. sequential unary calls.
//!
//! Plan Maestro §11.T6.3 AC: streaming throughput must be ≥ 1.5× the
//! equivalent unary call sequence. The bench runs an in-process gRPC server
//! that immediately echoes a small payload so the measurement isolates the
//! protocol overhead (HTTP/2 frame setup, header compression, etc.) from
//! any model latency.
//!
//! Run with:
//!
//!     cargo bench -p strata-ia-bridge --bench stream_vs_unary

use std::time::Duration;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use futures::StreamExt;
use strata_ia_bridge::proto::{
    ia_service_server::{IaService, IaServiceServer},
    BBox, Crop, FormulaResponse, ImageDescription, ImageResponse, OcrResponse, OcrResult,
    Provenance, StreamCrop, StreamResult, TableResponse,
};
use strata_ia_bridge::{BridgeClient, BridgeClientConfig};
use tokio::sync::oneshot;
use tonic::{transport::Server, Request, Response, Status, Streaming};

#[derive(Default)]
struct EchoIa;

#[tonic::async_trait]
impl IaService for EchoIa {
    async fn ocr_page(&self, _req: Request<Crop>) -> Result<Response<OcrResponse>, Status> {
        Ok(Response::new(canned_ocr()))
    }
    async fn extract_table(&self, _req: Request<Crop>) -> Result<Response<TableResponse>, Status> {
        Ok(Response::new(TableResponse {
            result: None,
            provenance: Some(canned_provenance()),
        }))
    }
    async fn describe_image(&self, _req: Request<Crop>) -> Result<Response<ImageResponse>, Status> {
        Ok(Response::new(ImageResponse {
            result: Some(ImageDescription {
                caption: "x".into(),
                description: "".into(),
                alt_text: "".into(),
                confidence: 0.5,
            }),
            provenance: Some(canned_provenance()),
        }))
    }
    async fn ocr_formula(&self, _req: Request<Crop>) -> Result<Response<FormulaResponse>, Status> {
        Ok(Response::new(FormulaResponse {
            result: None,
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
                yield StreamResult {
                    correlation_id: crop.correlation_id,
                    payload: Some(strata_ia_bridge::proto::stream_result::Payload::Ocr(canned_ocr())),
                };
            }
        };
        Ok(Response::new(Box::pin(stream)))
    }
}

fn canned_ocr() -> OcrResponse {
    OcrResponse { result: Some(OcrResult::default()), provenance: Some(canned_provenance()) }
}

fn canned_provenance() -> Provenance {
    Provenance {
        model_id: "bench".into(),
        backend: "echo".into(),
        latency_ms: 0,
        retries: 0,
        cache_hit: false,
    }
}

fn canned_crop() -> Crop {
    Crop {
        png_bytes: vec![0; 64].into(),
        dpi: 72,
        page_no: 1,
        bbox: Some(BBox { x0: 0.0, y0: 0.0, x1: 1.0, y1: 1.0 }),
        hint: "".into(),
    }
}

async fn spawn_server() -> (tokio::task::JoinHandle<()>, String, oneshot::Sender<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (tx, rx) = oneshot::channel::<()>();
    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(IaServiceServer::new(EchoIa))
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

fn bench(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    let (server_handle, endpoint, shutdown) = rt.block_on(spawn_server());
    let client = rt.block_on(async {
        // Give the server a beat to bind.
        tokio::time::sleep(Duration::from_millis(50)).await;
        BridgeClient::connect(BridgeClientConfig { endpoint: endpoint.clone(), ..Default::default() })
            .await
            .unwrap()
    });

    for batch in [16usize, 64, 256] {
        let mut group = c.benchmark_group("ia_bridge_throughput");
        group.throughput(Throughput::Elements(batch as u64));

        group.bench_with_input(BenchmarkId::new("unary", batch), &batch, |b, &batch| {
            b.to_async(&rt).iter(|| async {
                for i in 0..batch {
                    let _ = client.ocr_page(canned_crop()).await.unwrap();
                    std::hint::black_box(i);
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("stream", batch), &batch, |b, &batch| {
            b.to_async(&rt).iter(|| async {
                let crops = futures::stream::iter((0..batch).map(|i| StreamCrop {
                    correlation_id: format!("c-{i}"),
                    route: strata_ia_bridge::proto::TriageRoute::OcrPage as i32,
                    crop: Some(canned_crop()),
                }));
                let mut out = client.process_stream(crops).await.unwrap();
                let mut received = 0;
                while let Some(item) = out.next().await {
                    let _ = item.unwrap();
                    received += 1;
                }
                assert_eq!(received, batch);
            });
        });
        group.finish();
    }

    let _ = shutdown.send(());
    rt.block_on(async {
        let _ = server_handle.await;
    });
}

criterion_group! {
    name = streams;
    config = Criterion::default().sample_size(20).measurement_time(Duration::from_secs(8));
    targets = bench
}
criterion_main!(streams);
