//! Módulo: ia_bridge
//!
//! Descripción:
//! Conexión al bridge gRPC de IA y procesamiento de IaTasks.
//! Gestiona la conexión al microservicio Python (externo o embedded) y
//! envía/recibe crops en modo streaming bidireccional.

use std::collections::HashMap;

use futures::StreamExt;
use tracing::{debug, warn};

use strata_core::{BlockId, BlockType};
use strata_fusion::IaPayload;
use strata_ia_bridge::proto::stream_result::Payload;
use strata_ia_bridge::proto::{Crop, StreamCrop, TriageRoute as ProtoRoute};
use strata_ia_bridge::{BridgeClient, BridgeClientConfig, EmbeddedWorker, SpawnOptions};
use strata_triage::TriageRoute;

use crate::error::PipelineError;
use crate::options::ParsePipelineOptions;

/// Una tarea que debe escalarse a IA.
#[derive(Debug, Clone)]
pub struct IaTask {
    pub block_id: BlockId,
    pub block_type: BlockType,
    pub route: TriageRoute,
    pub png_bytes: Vec<u8>,
    pub hint: String,
    pub page_no: u32,
    pub dpi: u32,
}

/// Envía todas las tareas IA al bridge y retorna un mapa `BlockId → IaPayload`.
///
/// Si `tasks` está vacío, retorna un mapa vacío sin conectar al bridge.
///
/// # Errores
///
/// Retorna `PipelineError::IaUnavailable` si no se puede conectar al bridge,
/// o `PipelineError::IaRpc` si falla la llamada gRPC.
pub async fn run_ia_tasks(
    tasks: Vec<IaTask>,
    opts: &ParsePipelineOptions,
) -> Result<HashMap<BlockId, IaPayload>, PipelineError> {
    if tasks.is_empty() {
        return Ok(HashMap::new());
    }

    let (client, _worker) = connect_bridge(opts).await?;

    // Mapa de correlation_id → (BlockId, TriageRoute, BlockType).
    let corr_map: HashMap<String, (BlockId, TriageRoute, BlockType)> = tasks
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let id = format!("corr-{i}-{}", t.block_id);
            (id, (t.block_id, t.route, t.block_type.clone()))
        })
        .collect();

    // Construir stream de StreamCrops.
    let crops: Vec<StreamCrop> = tasks
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let corr_id = format!("corr-{i}-{}", t.block_id);
            let proto_route = triage_route_to_proto(t.route) as i32;
            StreamCrop {
                correlation_id: corr_id,
                route: proto_route,
                crop: Some(Crop {
                    png_bytes: t.png_bytes.clone(),
                    dpi: t.dpi,
                    page_no: t.page_no,
                    bbox: None,
                    hint: t.hint.clone(),
                }),
            }
        })
        .collect();

    let crop_stream = futures::stream::iter(crops);

    let mut response_stream = client
        .process_stream(crop_stream)
        .await
        .map_err(|e| PipelineError::IaRpc(e.to_string()))?;

    let mut results: HashMap<BlockId, IaPayload> = HashMap::new();

    while let Some(item) = response_stream.next().await {
        let stream_result = match item {
            Ok(r) => r,
            Err(e) => {
                warn!(error = %e, "IA stream error, saltando item");
                continue;
            }
        };

        let corr_id = &stream_result.correlation_id;
        let Some((block_id, route, _block_type)) = corr_map.get(corr_id) else {
            warn!(
                corr_id = corr_id.as_str(),
                "correlation_id desconocido en respuesta IA"
            );
            continue;
        };

        let payload = match stream_result.payload {
            Some(Payload::Ocr(resp)) => crate::mappers::ocr_to_payload(&resp),
            Some(Payload::Table(resp)) => crate::mappers::table_to_payload(&resp),
            Some(Payload::Image(resp)) => crate::mappers::image_to_payload(&resp),
            Some(Payload::Formula(resp)) => crate::mappers::formula_to_payload(&resp),
            Some(Payload::Error(e)) => {
                warn!(
                    block_id = %block_id,
                    error_code = e.code,
                    error_msg = e.message.as_str(),
                    route = ?route,
                    "IA retornó error para el bloque"
                );
                None
            }
            None => None,
        };

        if let Some(p) = payload {
            debug!(block_id = %block_id, "payload IA recibido");
            results.insert(*block_id, p);
        }
    }

    Ok(results)
}

/// Conecta al bridge: usa endpoint externo si está configurado; si no, lanza un `EmbeddedWorker`.
async fn connect_bridge(
    opts: &ParsePipelineOptions,
) -> Result<(BridgeClient, Option<EmbeddedWorker>), PipelineError> {
    if let Some(ref endpoint) = opts.ia_grpc_endpoint {
        let cfg = BridgeClientConfig {
            endpoint: endpoint.clone(),
            ..Default::default()
        };
        let client = BridgeClient::connect(cfg)
            .await
            .map_err(|e| PipelineError::IaUnavailable(e.to_string()))?;
        return Ok((client, None));
    }

    // Lanzar embedded worker Python.
    let spawn_opts = SpawnOptions {
        health_timeout: std::time::Duration::from_secs(15),
        env: Some(vec![(
            "STRATA_IA_OLLAMA_ENDPOINT".into(),
            opts.ollama_endpoint.clone(),
        )]),
        ..Default::default()
    };
    let worker = EmbeddedWorker::spawn_with(spawn_opts)
        .await
        .map_err(|e| PipelineError::IaUnavailable(format!("embedded worker falló: {e}")))?;
    let client = worker
        .connect_client()
        .await
        .map_err(|e| PipelineError::IaUnavailable(e.to_string()))?;
    Ok((client, Some(worker)))
}

/// Mapea la ruta de `TriageRoute` (Rust) a `TriageRoute` proto (prost).
fn triage_route_to_proto(route: TriageRoute) -> ProtoRoute {
    match route {
        TriageRoute::OcrFullPage => ProtoRoute::OcrPage,
        TriageRoute::VlmTable => ProtoRoute::Table,
        TriageRoute::VlmImage => ProtoRoute::Image,
        TriageRoute::VlmFormula => ProtoRoute::Formula,
        TriageRoute::Native => ProtoRoute::Unspecified,
    }
}
