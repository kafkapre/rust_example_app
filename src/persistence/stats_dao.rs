use anyhow::Result;
use async_trait::async_trait;

use crate::model::ml_dataset_stats::MlDatasetStats;

pub type StatsDao = &'static dyn StatsDaoLike;

#[async_trait]
pub trait StatsDaoLike: Sync {
  async fn fetch_stats(&self, matches_count: i32) -> Result<Vec<MlDatasetStats>>;

  async fn store_stats(&self, stats: MlDatasetStats) -> Result<()>;

  async fn test_fn(&self) -> Result<Vec<MlDatasetStats>> {
    self.fetch_stats(1).await
  }
}
