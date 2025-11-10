use anyhow::{Context, Result};
use std::env;

#[derive(Debug)]
pub struct AppConfig {
    db_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            db_url: env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
        })
    }

    pub fn db_url(&self) -> &str {
        &self.db_url
    }
}
