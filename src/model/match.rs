use chrono::{DateTime, Utc};

use crate::model::gold_value::GoldValue;

#[derive(Clone, PartialEq, Debug)] // copy is not enabled
pub enum Match {
  Csgo(CsgoMatch),
  Dota2 { started_at: DateTime<Utc>, values: Dota2Match },
  Lol { started_at: DateTime<Utc>, values: LolMatch },
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct CsgoMatch {
  pub started_at: DateTime<Utc>,
  pub home: u64,
  pub away: u64,
  pub rounds: u32,
  pub home_won: Option<bool>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Dota2Match {
  pub started_at: DateTime<Utc>,
  pub gold: GoldValue,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct LolMatch {
  pub started_at: DateTime<Utc>,
  pub red: u32,
  pub blue: u32,
}

impl Match {
  pub fn started_at(&self) -> DateTime<Utc> { // not a best approach
    match self {
      Self::Csgo(values) => values.started_at,
      Self::Dota2 { values, .. } => values.started_at,
      Self::Lol { values, .. } => values.started_at,
    }
  }
}
