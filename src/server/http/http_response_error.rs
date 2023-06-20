use actix_web::{error, HttpResponse};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum HttpResponseError {
  #[error("Internal error: {msg:?}")]
  InternalError {
    msg: String,

    #[source]
    source: anyhow::Error,
  },
}

impl error::ResponseError for HttpResponseError {
  fn status_code(&self) -> StatusCode {
    match *self {
      Self::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }

  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status_code())
      .insert_header(ContentType::html())
      .body(self.to_string())
  }
}
