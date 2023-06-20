use chrono::{DateTime, Utc};

pub struct ProcessorState {
  pub threshold: DateTime<Utc>,
}

impl Default for ProcessorState {
  fn default() -> Self {
    Self {
      threshold: Utc::now(),
    }
  }
}
