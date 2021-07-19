use deadpool_postgres::{Config, Pool};
use serde::Deserialize;

pub mod db;

#[derive(Debug, Clone, Deserialize)]
pub struct DbConfig{
    config: Config
}

impl DbConfig{

    pub fn from_env() -> anyhow::Result<Self>{
        let mut config = config::Config::new();
        dotenv::vars().filter(|var| var.0.starts_with("PG.")).for_each(|var| {
            config.set(&var.0[3..].to_lowercase(), var.1.clone()).unwrap();
        });

        Ok(DbConfig{config:config.try_into()?})
    }

    pub fn create_pool(&mut self) -> anyhow::Result<Pool>{
        Ok(self.config.create_pool(tokio_postgres::NoTls)?)
    }
}
