#![allow(clippy::missing_errors_doc, clippy::must_use_candidate, clippy::module_name_repetitions, clippy::module_inception)]
#![allow(clippy::redundant_closure_for_method_calls, clippy::manual_non_exhaustive)]
#![deny(unused_must_use)]

use std::sync::{Arc, OnceLock};

use ::config::Config;
use actix_web::rt;
use anyhow::{Context, Result};
use tracing::{info, Level};
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::configuration::app_config::{AppConfig, RawAppConfig};
use crate::monitoring::monitoring_example::data_collector;
use crate::monitoring::register_custom_metrics;
use crate::monitoring::tracing_example::trace_example;
use crate::persistence::ml_dataset_dao::MlDatasetDaoLike;
use crate::persistence::numpy_files::ml_dataset_dao_numpy_files::MlDatasetDaoNumpyFiles;
use crate::persistence::sqlx::stats_dao_sqlx::StatsDaoSqlx;
use crate::persistence::stats_dao::StatsDaoLike;
use crate::processor::processor::Processor;
use crate::server::http::http_server::HttpServer;

mod server;
mod configuration;
mod persistence;
mod model;
mod examples;
mod processor;
mod monitoring;

static CONFIG_CELL: OnceLock<AppConfig> = OnceLock::new();
static STATS_DAO_CELL: OnceLock<StatsDaoSqlx> = OnceLock::new();

// #[tokio::main]
#[actix_web::main]
async fn main() -> Result<()> {
  let tracer = opentelemetry_jaeger::new_agent_pipeline()
    .with_endpoint("127.0.0.1:6831")
    .with_service_name("report_example")
    .install_simple()
    .context("init open_telemetry_jaeger failed")?;

  tracing_subscriber::registry()
    .with(tracing_opentelemetry::layer().with_tracer(tracer))
    .with(
      fmt::Layer::new()
        .with_writer(std::io::stdout.with_max_level(Level::INFO))
        .with_target(false)
    )
    .try_init()
    .context("init tracing registry failed")?;

  let conf_raw = Config::builder()
    .add_source(config::File::with_name("config/config.yaml"))
    .build()
    .and_then(Config::try_deserialize::<RawAppConfig>)
    .context("app config processing failed")?;

  info!("#");
  info!("# ---------------------------------------------------");
  info!("# Rust Example App - STARTING");
  info!("# ---------------------------------------------------");
  info!("#");

  register_custom_metrics();
  tokio::task::spawn(data_collector()); // random monitoring data
  trace_example(); // random monitoring data

  let conf = CONFIG_CELL.get_or_init(|| conf_raw.app_config);
  let stats_dao: &'static dyn StatsDaoLike = {
    let dao = StatsDaoSqlx::new(&conf.postgres_db).await?;
    STATS_DAO_CELL.get_or_init(|| dao)
  };
  let ml_dataset_dao: Arc<dyn MlDatasetDaoLike + Send + Sync> = Arc::new(MlDatasetDaoNumpyFiles::new()); // overkill

  let processor = Arc::new(Processor::new(stats_dao, ml_dataset_dao));

  let http_server = HttpServer::run()?;
  let http_server_handle = http_server.handle();
  rt::spawn(http_server);

  info!("Starting http server on 0.0.0.0:3030");

  server::grpc::grpc_server::connect(processor, &conf.grpc_server).await?;

  http_server_handle.stop(false).await;

  Ok(())
}
