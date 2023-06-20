use std::time::Duration;

use rand::{Rng, thread_rng};

use crate::monitoring::{RESPONSE_CODE_COLLECTOR, RESPONSE_TIME_COLLECTOR};

const ENVS: &[&str] = &["testing", "production"];

pub async fn data_collector() {
  let mut collect_interval = tokio::time::interval(Duration::from_millis(10));
  loop {
    collect_interval.tick().await;
    let mut rng = thread_rng();
    let response_time: f64 = rng.gen_range(0.1..10.0);
    let response_code: usize = rng.gen_range(100..599);
    let env_index: usize = rng.gen_range(0..2);

    track_status_code(response_code, ENVS.get(env_index).expect("exists"));
    track_request_time(response_time, ENVS.get(env_index).expect("exists"));
  }
}

fn track_request_time(response_time: f64, env: &str) {
  RESPONSE_TIME_COLLECTOR
    .with_label_values(&[env])
    .observe(response_time);
}

fn track_status_code(status_code: usize, env: &str) {
  match status_code {
    500..=599 => RESPONSE_CODE_COLLECTOR
      .with_label_values(&[env, &status_code.to_string(), "500"])
      .inc(),
    400..=499 => RESPONSE_CODE_COLLECTOR
      .with_label_values(&[env, &status_code.to_string(), "400"])
      .inc(),
    300..=399 => RESPONSE_CODE_COLLECTOR
      .with_label_values(&[env, &status_code.to_string(), "300"])
      .inc(),
    200..=299 => RESPONSE_CODE_COLLECTOR
      .with_label_values(&[env, &status_code.to_string(), "200"])
      .inc(),
    100..=199 => RESPONSE_CODE_COLLECTOR
      .with_label_values(&[env, &status_code.to_string(), "100"])
      .inc(),
    _ => ()
  };
}
