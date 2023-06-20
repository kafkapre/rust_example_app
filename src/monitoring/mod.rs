use anyhow::{Context, Result};
use lazy_static::lazy_static;
use prometheus::{
  HistogramOpts, HistogramVec, IntCounter, IntCounterVec, IntGauge, Opts, Registry,
};


pub mod monitoring_example;
pub mod tracing_example;

lazy_static! {
    pub static ref MONITORING_REGISTRY: Registry = Registry::new();

    pub static ref INCOMING_REQUESTS: IntCounter =
        IntCounter::new("incoming_requests", "Incoming Requests").expect("metric can be created");

    pub static ref CONNECTED_CLIENTS: IntGauge =
        IntGauge::new("connected_clients", "Connected Clients").expect("metric can be created");

    pub static ref RESPONSE_CODE_COLLECTOR: IntCounterVec = IntCounterVec::new(
        Opts::new("response_code", "Response Codes"),
        &["env", "statuscode", "type"]
    )
    .expect("metric can be created");

    pub static ref RESPONSE_TIME_COLLECTOR: HistogramVec = HistogramVec::new(
        HistogramOpts::new("response_time", "Response Times"),
        &["env"]
    )
    .expect("metric can be created");
}

pub fn register_custom_metrics() {
  MONITORING_REGISTRY
    .register(Box::new(INCOMING_REQUESTS.clone()))
    .expect("collector can be registered");

  MONITORING_REGISTRY
    .register(Box::new(CONNECTED_CLIENTS.clone()))
    .expect("collector can be registered");

  MONITORING_REGISTRY
    .register(Box::new(RESPONSE_CODE_COLLECTOR.clone()))
    .expect("collector can be registered");

  MONITORING_REGISTRY
    .register(Box::new(RESPONSE_TIME_COLLECTOR.clone()))
    .expect("collector can be registered");
}

pub fn collect_monitoring_data_for_prometheus() -> Result<String> {
  use prometheus::Encoder;
  let encoder = prometheus::TextEncoder::new();

  let mut buffer = Vec::new();
  encoder.encode(&MONITORING_REGISTRY.gather(), &mut buffer)
    .context("could not encode custom metrics")?;

  let mut res = String::from_utf8(buffer.clone())
    .context("custom metrics could not be from_utf8'd")?;

  buffer.clear();

  let mut buffer = Vec::new();
  encoder.encode(&prometheus::gather(), &mut buffer)
    .context("could not encode prometheus metrics")?;
  let res_custom = String::from_utf8(buffer.clone())
    .context("prometheus metrics could not be from_utf8'd")?;

  buffer.clear();

  res.push_str(&res_custom);

  Ok(res)
}
