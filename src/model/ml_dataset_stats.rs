use crate::model::gold_value::GoldValue;

#[derive(Copy, Clone, Debug)]
pub struct MlDatasetStats {
  pub matches_count: usize,
  pub csgo_home_won: usize,
  pub gold_sum: GoldValue,
}

impl Default for MlDatasetStats {
  fn default() -> Self {
    MlDatasetStats {
      matches_count: 0,
      csgo_home_won: 0,
      gold_sum: GoldValue::ZERO,
    }
  }
}
