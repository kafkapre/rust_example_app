use std::sync::Arc;
use async_trait::async_trait;
use crate::model::ml_dataset::MlDataset;
use anyhow::Result;

pub type MlDatasetDao = Arc<dyn MlDatasetDaoLike  + Send + Sync>;

#[async_trait]
pub trait MlDatasetDaoLike {
  async fn store_ml_data_set(&self, dataset: MlDataset) -> Result<()>;
}

