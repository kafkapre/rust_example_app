use actix_web::{App, get, HttpRequest, middleware::Logger};
use actix_web::dev::Server;
use anyhow::Result;
use tracing::error;

use crate::monitoring::collect_monitoring_data_for_prometheus;
use crate::server::http::http_response_error::HttpResponseError;

pub struct HttpServer {}

impl HttpServer {

  pub fn run() -> Result<Server> {
    let server = actix_web::HttpServer::new(|| {
      App::new()
        .wrap(Logger::new(r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %D ms"#))
        .service(health_check)
        .service(metrics_handler)
    })
      .bind(("127.0.0.1", 3030))?
      .run();

    Ok(server)
  }
}

#[allow(clippy::unused_async)]
#[get("/health-check")]
async fn health_check(_: HttpRequest) -> &'static str {
  "App is OK"
}

#[allow(clippy::unused_async)]
#[get("/metrics")]
async fn metrics_handler(_: HttpRequest) -> Result<String, HttpResponseError> {
  collect_monitoring_data_for_prometheus()
    .map_err(|err| {
      error!("{}", err);
      HttpResponseError::InternalError { msg: err.to_string(), source: err }
    })
}
