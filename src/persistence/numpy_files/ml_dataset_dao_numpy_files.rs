use std::path::Path;
use std::sync::Arc;

use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use pyo3::{PyResult, Python, ToPyObject};
use pyo3::types::IntoPyDict;

use crate::model::ml_dataset::MlDataset;
use crate::persistence::ml_dataset_dao::MlDatasetDaoLike;

pub struct MlDatasetDaoNumpyFiles {
  // - pool with one primitive worker equivalent
  // - keep in mind that tokio mutex is used here
  idx: Arc<tokio::sync::Mutex<u32>>,
}

impl MlDatasetDaoNumpyFiles {
  pub fn new() -> Self {
    MlDatasetDaoNumpyFiles {
      idx: Arc::new(tokio::sync::Mutex::new(0)),
    }
  }
}

#[async_trait]
impl MlDatasetDaoLike for MlDatasetDaoNumpyFiles {
  async fn store_ml_data_set(&self, dataset: MlDataset) -> Result<()> {
    let mut idx = self.idx.lock().await;
    *idx += 1;

    let dir = Path::new("./persistence/numpy");
    let file = format!("data_{}.npy", *idx);

    let mut data = Vec::with_capacity(dataset.csgo_matches.len());
    for m in &dataset.csgo_matches {
      data.push(vec![m.home as f32, m.away as f32]);
    }

    Self::store_2d_vec(dir, file, data)
  }
}

impl MlDatasetDaoNumpyFiles {
  pub fn store_2d_vec(dir: &Path, file: impl AsRef<str>, data: Vec<Vec<f32>>) -> Result<()> {
    Self::store_data(dir, file, data)
  }

  fn store_data<T: ToPyObject>(dir: &Path, file: impl AsRef<str>, data: T) -> Result<()> {
    if !dir.exists() {
      bail!("directory [{}] does not exists", dir.display());
    }

    let path = dir.join(Path::new(file.as_ref()));
    if path.exists() {
      bail!("file [{}] already does. Override is not supported", path.display());
    }

    let res: PyResult<()> = Python::with_gil(|py| {
      let np = py.import("numpy")?;
      let locals = [("np", np), ].into_py_dict(py);
      locals.set_item("path", path.display().to_string())?;
      locals.set_item("data", data)?;

      py.eval(
        r#"np.save(path, np.array(data, dtype=np.single))"#,
        Some(locals),
        None,
      )?;

      Ok(())
    });

    res.context("store numpy data failed")
  }
}
