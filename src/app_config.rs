use anyhow::{Context, Result};
use std::env;

#[derive(Debug)]
pub struct AppConfig {
    db_url: String,
    server_port: u16,
    server_address: String
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            db_url: env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
            server_address: env::var("SERVER_ADDRESS").context("SERVER_ADDRESS must be set")?
        })
    }

    pub fn db_url(&self) -> &str {
        &self.db_url
    }

    pub fn server_port(&self) -> &u16 {
        &self.server_port
    }

    pub fn full_server_address(&self) -> String {
       format!("{}:{}", &self.server_address, &self.server_port) 
    }
}
