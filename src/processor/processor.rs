use std::cell::Cell;
use std::sync::Mutex;

use anyhow::bail;
use anyhow::Result;

use crate::model::ml_dataset::MlDataset;
use crate::model::r#match::Match;
use crate::persistence::ml_dataset_dao::MlDatasetDao;
use crate::persistence::stats_dao::StatsDao;
use crate::processor::processor_state::ProcessorState;

pub struct Processor {
  state: Mutex<Cell<ProcessorState>>,
  stats_dao: StatsDao,
  ml_dataset_dao: MlDatasetDao,
  _private: (),
}

impl Processor {
  pub fn new(
    stats_dao: StatsDao,
    ml_dataset_dao: MlDatasetDao,
  ) -> Self {
    Self {
      state: Mutex::new(Cell::new(ProcessorState::default())),
      stats_dao,
      ml_dataset_dao,
      _private: (),
    }
  }


  pub fn set_state(&self, new_state: ProcessorState) -> Result<()> {
    let Ok(state) = self.state.lock() else {
      bail!("unlock failed")
    };

    state.set(new_state);

    Ok(())
  }

  pub async fn process(&self, matches: Vec<Match>) -> Result<()> {
    let state = self.get_state()?;

    // VS

    // let Ok(state) = self.state.lock() else {      // NOTE: tokio mutexes would fix it, but performance would be hurt
    //   bail!("unlock failed")
    // };

    let dataset = MlDataset::new(state.threshold, matches);

    self.stats_dao.store_stats(dataset.stats).await?;
    self.ml_dataset_dao.store_ml_data_set(dataset).await?;

    Ok(())
  }

  pub fn get_state(&self) -> Result<ProcessorState> {
    let Ok(state) = self.state.lock() else {
      bail!("unlock failed")
    };

    Ok(state.take())
  }
}
