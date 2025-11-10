use anyhow::{Context, Result};
use std::env;

#[derive(Debug)]
pub struct AppConfig {
    db_url: String,
    port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            db_url: env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
        })
    }

    pub fn db_url(&self) -> &str {
        &self.db_url
    }

    pub fn port(&self) -> &u16 {
        &self.port
    }
}
