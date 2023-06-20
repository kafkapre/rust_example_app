use chrono::{DateTime, Utc};

use crate::model::ml_dataset_stats::MlDatasetStats;
use crate::model::r#match::{CsgoMatch, Dota2Match, LolMatch, Match};

#[derive(Clone, Debug)]
pub struct MlDataset {
  pub timestamp_threshold: DateTime<Utc>,
  pub csgo_matches: Vec<CsgoMatch>,
  pub dota2_matches: Vec<Dota2Match>,
  pub lol_matches: Vec<LolMatch>,
  pub stats: MlDatasetStats,
  _private: (),
}

impl MlDataset {
  pub fn new(timestamp_threshold: DateTime<Utc>, matches: Vec<Match>) -> Self {
    let stats = Self::compute_stats_mutable_approach(&matches); // show why stats is defined here

    let mut csgo_matches: Vec<CsgoMatch> = Vec::with_capacity(matches.len()); // type is derived
    let mut dota2_matches: Vec<Dota2Match> = vec![]; // type is derived
    let mut lol_matches: Vec<LolMatch> = vec![]; // type is derived explicitly
    for m in matches.iter().filter(|m| m.started_at() < timestamp_threshold) {
      match m {
        Match::Csgo(m) => csgo_matches.push(*m),
        Match::Dota2 { values, .. } => dota2_matches.push(*values),
        Match::Lol { values, .. } => lol_matches.push(*values),
      }
    }

    Self { timestamp_threshold, csgo_matches, dota2_matches, lol_matches, stats, _private: () }
  }

  fn compute_stats_mutable_approach(matches: &Vec<Match>) -> MlDatasetStats {
    let mut stats_mutable = MlDatasetStats::default();
    for m in matches {
      stats_mutable.matches_count += 1;
      match m {
        Match::Csgo(CsgoMatch { home_won: Some(true), .. }) => {
          stats_mutable.csgo_home_won += 1
        }
        Match::Csgo(_) => (), // skip
        Match::Dota2 { started_at, values, .. } if *started_at < Utc::now() => {
          stats_mutable.gold_sum += values.gold;
        }
        Match::Dota2 { .. } => (),
        Match::Lol { .. } => (),
      }
    };

    stats_mutable
  }

  #[allow(dead_code)]
  fn compute_stats_immutable_approach(matches: &Vec<Match>) -> MlDatasetStats {
    matches
      .iter()
      .fold(MlDatasetStats::default(), |stats, m| {
        match m {
          Match::Csgo(CsgoMatch { home_won: Some(true), .. }) => {
            MlDatasetStats {
              matches_count: stats.matches_count + 1,
              csgo_home_won: stats.csgo_home_won + 1,
              ..stats
            }
          }
          Match::Dota2 { started_at, values, .. } if started_at.lt(&Utc::now()) => {
            MlDatasetStats {
              matches_count: stats.matches_count + 1,
              gold_sum: stats.gold_sum + values.gold,
              ..stats
            }
          }
          Match::Csgo(_) | Match::Dota2 { .. } | Match::Lol { .. } => {
            MlDatasetStats {
              matches_count: stats.matches_count + 1,
              ..stats
            }
          }
        }
      })
  }
}
