use anyhow::anyhow;
use rust_decimal::prelude::ToPrimitive;
use crate::model::gold_value::GoldValue;

use crate::model::ml_dataset_stats::MlDatasetStats;

#[derive(sqlx::FromRow)]
pub struct MlDatasetStatsSqlx {
  pub matches_count: i64,
  // catch MlDatasetStats may be u64
  pub csgo_home_won: i32,
  pub gold_sum: f32,
}

impl TryFrom<MlDatasetStats> for MlDatasetStatsSqlx {
  type Error = anyhow::Error;

  fn try_from(value: MlDatasetStats) -> Result<Self, Self::Error> {
    value.csgo_home_won.to_i32()
      .ok_or_else(|| anyhow!("csgo_home_won conversion failed: {}",  value.csgo_home_won))
      .map(|csgo_home_won| {
        Self {
          matches_count: value.matches_count as i64, // unsafe cast
          csgo_home_won,
          gold_sum: value.gold_sum.value,
        }
      })
  }
}

impl Into<MlDatasetStats> for MlDatasetStatsSqlx {
  fn into(self) -> MlDatasetStats {
    MlDatasetStats {
      matches_count: self.matches_count as usize,
      csgo_home_won: self.csgo_home_won as usize,
      gold_sum: GoldValue::new(self.gold_sum).unwrap(), // never use unwrap()
    }
  }
}
