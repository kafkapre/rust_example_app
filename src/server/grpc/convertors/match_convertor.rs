use anyhow::bail;

use rust_example_app::grpc::r#match::MatchType;

use crate::model::gold_value::GoldValue;
use crate::model::r#match::{CsgoMatch, Dota2Match, LolMatch, Match};
use crate::server::grpc::convertors::timestamp_convertor::utc_from_str;

impl TryInto<Match> for &rust_example_app::grpc::Match {
  type Error = anyhow::Error;

  fn try_into(self) -> Result<Match, Self::Error> {
    let res = match &self.match_type {
      Some(MatchType::Csgo(csgo)) => {
        let started_at = utc_from_str(csgo.started_at.as_str())?;
        Match::Csgo(
          CsgoMatch {
            started_at,
            home: csgo.home,
            away: csgo.away,
            rounds: csgo.rounds,
            home_won: csgo.home_won,
          }
        )
      }
      Some(MatchType::Dota2(dota)) => {
        let started_at = utc_from_str(dota.started_at.as_str())?;
        Match::Dota2 { started_at, values: Dota2Match { started_at, gold: GoldValue::new(dota.gold)? } }
      }
      Some(MatchType::Lol(lol)) => {
        let started_at = utc_from_str(lol.started_at.as_str())?;
        let values = LolMatch {
          started_at,
          red: lol.red,
          blue: lol.blue,
        };
        Match::Lol { started_at, values }
      }
      _ => bail!("invalid input")
    };

    Ok(res)
  }
}
