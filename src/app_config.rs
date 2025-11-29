use anyhow::{Context, Result, anyhow};
use std::env;

#[derive(Debug)]
pub struct AppConfig {
    db_url: String,
    server_port: u16,
    host: String,
    jwt_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        let db_url =
            env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
        if !db_url.starts_with("postgres://postgres:") {
            Err(anyhow!("DATABASE_URL must start with postgres://postgres:"))?
        }

        let jwt_secret =
            env::var("JWT_SECRET").context("JWT_SECRET must be set")?;

        Ok(Self {
            db_url,
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
            host: env::var("HOST").context("HOST must be set")?,
            jwt_secret,
        })
    }

    pub fn db_url(&self) -> &str {
        &self.db_url
    }

    pub fn server_port(&self) -> &u16 {
        &self.server_port
    }

    pub fn full_server_address(&self) -> String {
        format!("{}:{}", &self.host, &self.server_port)
    }

    pub fn jwt_secret(&self) -> &String {
        &self.jwt_secret
    }
}
