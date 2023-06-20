use ::sqlx::{Pool, Postgres};
use ::sqlx::postgres::PgPoolOptions;
use anyhow::{Context, Result};
use async_trait::async_trait;

use crate::configuration::app_config::PostgresConfig;
use crate::model::ml_dataset_stats::MlDatasetStats;
use crate::persistence::sqlx::model::ml_dataset_stats_sqlx::MlDatasetStatsSqlx;
use crate::persistence::stats_dao::StatsDaoLike;

#[derive(Clone, Debug)]
pub struct StatsDaoSqlx {
  pool: Pool<Postgres>,
}

impl StatsDaoSqlx {
  pub async fn new(config: &PostgresConfig) -> Result<Self> {
    let pool = PgPoolOptions::new()
      .max_connections(config.max_connections)
      .connect(Self::conf_to_url(config).as_str()).await
      .context("connection to StatsDaoSqlx failed")?;

    Ok(Self { pool })
  }

  fn conf_to_url(conf: &PostgresConfig) -> String {
    format!("postgres://{}:{}@{}:{}/{}", conf.user, conf.password, conf.host, conf.port, conf.db_name)
  }
}

#[async_trait]
impl StatsDaoLike for StatsDaoSqlx {
  async fn fetch_stats(&self, matches_count: i32) -> Result<Vec<MlDatasetStats>> {
    let query = r"SELECT matches_count, csgo_home_won, gold_sum \
      FROM public.stats \
      WHERE matches_count >= $1\
      ORDER BY gold_sum";

    let values = ::sqlx::query_as::<_, MlDatasetStatsSqlx>(query)
      .bind(matches_count)
      .fetch_all(&self.pool)
      .await?
      .into_iter()
      .map(|t| t.into())
      .collect::<Vec<MlDatasetStats>>();

    Ok(values)
  }

  async fn store_stats(&self, stats: MlDatasetStats) -> Result<()> {
    let query = format!(
      "INSERT INTO {} (matches_count, csgo_home_won, gold_sum) \
       VALUES ( $1, $2, $3 )",
      "public.stats", // replace {}
    );

    let stats = MlDatasetStatsSqlx::try_from(stats).unwrap();
    sqlx::query(query.as_str())
      .bind(stats.matches_count)
      .bind(stats.csgo_home_won)
      .bind(stats.gold_sum)
      .execute(&self.pool)
      .await
      .map(|_| ())
      .context("store row in transaction failed")
  }
}
