use tracing::{span, trace};
use tracing_attributes::instrument;

pub fn trace_example() {
  {
    let root = span!(tracing::Level::INFO, "app_start", work_units = 2);
    let _enter = root.enter();

    let work_result = expensive_work();

    span!(tracing::Level::INFO, "faster_work").in_scope(|| std::thread::sleep(std::time::Duration::from_millis(10)));
    trace!("status: {}", work_result);
  }

  {
    let root = span!(tracing::Level::INFO, "xxxx", work_units = 2);
    let _enter = root.enter();

    let work_result = expensive_work_aaa();

    span!(tracing::Level::INFO, "aaaa").in_scope(|| std::thread::sleep(std::time::Duration::from_millis(10)));
    trace!("status: {}", work_result);
  }
}

#[instrument]
#[inline]
fn expensive_work() -> &'static str {
  span!(tracing::Level::INFO, "expensive_step_1")
    .in_scope(|| std::thread::sleep(std::time::Duration::from_millis(25)));
  span!(tracing::Level::INFO, "expensive_step_2")
    .in_scope(|| std::thread::sleep(std::time::Duration::from_millis(25)));

  "success"
}

#[instrument]
#[inline]
fn expensive_work_aaa() -> &'static str {
  span!(tracing::Level::INFO, "aaa_step_1")
    .in_scope(|| std::thread::sleep(std::time::Duration::from_millis(25)));
  span!(tracing::Level::INFO, "aaaa_step_2")
    .in_scope(|| std::thread::sleep(std::time::Duration::from_millis(25)));

  "success"
}
