use std::ops::{Add, AddAssign};

use anyhow::bail;
use anyhow::Result;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GoldValue {
  pub value: f32,
  _private: (),
}

impl GoldValue {
  pub const ZERO: GoldValue = GoldValue { value: 0.0, _private: () };

  pub fn new(value: f32) -> Result<GoldValue> {
    if value < 0.0 {
      bail!("gold value [{}] cannot be negative", value)
    }
    Ok(GoldValue { value, _private: () })
  }
}


impl Add for GoldValue {
  type Output = GoldValue;

  fn add(self, other: Self) -> Self::Output {
    GoldValue { value: self.value + other.value, _private: () }
  }
}

impl AddAssign for GoldValue {
  fn add_assign(&mut self, other: Self) {
    self.value += other.value;
  }
}
