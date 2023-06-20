use std::sync::Arc;

use anyhow::{Context, Result};
use tonic::{Request, Response, Status, transport::Server};
use tonic::Code::Internal;
use tracing::info;

use rust_example_app::grpc::{SetProcessorThresholdRequest, SetProcessorThresholdResponse, StoreMatchesResultsRequest, StoreMatchesResultsResponse};
use rust_example_app::grpc::rust_example_api_server::{RustExampleApi, RustExampleApiServer};

use crate::configuration::app_config::GrpcServerConfig;
use crate::model::r#match::Match;
use crate::processor::processor::Processor;
use crate::processor::processor_state::ProcessorState;
use crate::server::grpc::convertors::timestamp_convertor::utc_from_str;

// !!! GRPC a Goland -  If generated grpc structs are not clickable in Goland, then check
// [https://github.com/hyperium/tonic#getting-started] pres 'CMD + SHIFT + A' and
// write 'Experimental features' then enable 'org.rust.cargo.evaluate.build.scripts' a restart
// Goland

pub async fn connect(
  processor: Arc<Processor>,
  conf: &GrpcServerConfig,
) -> Result<()> {
  let addr = (format!("{}:{}", conf.host, conf.port)).parse()?;
  let server = GrpcServerImpl { processor };

  info!("Starting gRPC server on {}", &addr);
  Server::builder()
    .add_service(RustExampleApiServer::new(server))
    .serve(addr)
    .await
    .context("server start failed")
}

// Implement the service skeleton for the "RustExampleApi" service
// defined in the proto
pub struct GrpcServerImpl {
  processor: Arc<Processor>,
}

// Implement the service function(s) defined in the proto
#[tonic::async_trait]
impl RustExampleApi for GrpcServerImpl {
  async fn store_matches_results(
    &self,
    request: Request<StoreMatchesResultsRequest>,
  ) -> Result<Response<StoreMatchesResultsResponse>, Status> {
    let matches: Result<Vec<Match>> = request.get_ref().matches.iter()
      .map(|m| m.try_into())
      .collect();

    let matches = matches
      .map_err(|e| Status::new(Internal, e.to_string()))?;

    self.processor.process(matches)
      .await
      .map_err(|e| Status::new(Internal, e.to_string()))?;

    Ok(
      Response::new(
        StoreMatchesResultsResponse {}
      )
    )
  }

  async fn set_processor_threshold(
    &self,
    request: Request<SetProcessorThresholdRequest>,
  ) -> Result<Response<SetProcessorThresholdResponse>, Status> {
    let threshold = utc_from_str(request.get_ref().threshold.as_str())
      .map_err(|e| Status::new(Internal, e.to_string()))?;

    self.processor.set_state(ProcessorState { threshold })
      .map_err(|e| Status::new(Internal, e.to_string()))?;

    Ok(
      Response::new(
        SetProcessorThresholdResponse {}
      )
    )
  }
}
