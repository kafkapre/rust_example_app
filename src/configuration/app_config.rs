use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct RawAppConfig {
  #[serde(rename = "rust-example-app")]
  pub app_config: AppConfig,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct AppConfig {
  pub grpc_server: GrpcServerConfig,
  pub postgres_db: PostgresConfig,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct GrpcServerConfig {
  pub host: String,
  pub port: u32,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct PostgresConfig {
  pub host: String,
  pub port: u32,
  pub db_name: String,
  pub user: String,
  pub password: String,
  pub max_connections: u32,
}
