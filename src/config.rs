use std::fs;

use anyhow::{Context, Error};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub log_level: String,
    pub interval: u64,
    pub metrics_host: String,
    pub metrics_port: u16,

    pub tracing: Tracing,
}

#[derive(Debug, Deserialize)]
pub struct Tracing {
    pub events: Events,
}

#[derive(Debug, Deserialize)]
pub struct Events {
    pub sys_enter: bool,
}

impl Config {
    pub fn metrics_address(&self) -> String {
        format!("{}:{}", self.metrics_host, self.metrics_port)
    }
}

pub fn from_file(path: &str) -> Result<Config, Error> {
    let data = fs::read_to_string(path).context("error opening config file")?;
    let config: Config = parse(data.as_str())?;

    Ok(config)
}

fn parse(data: &str) -> Result<Config, Error> {
    let config: Config = toml::from_str(data).context("error parsing config")?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that parsing config with missing fields returns an error.
    #[test]
    fn test_parse_error() {
        let toml_str = r#"
            does_not_exist = true
        "#;

        let config = parse(toml_str);
        assert!(config.is_err());
    }

    // Test that parsing a valid config file does not return an error.
    #[test]
    fn test_parse_success() {
        let toml_str = r#"
            log_level = "info"
            interval = 10
            metrics_host = "localhost"
            metrics_port = 9090

            [tracing]

            [tracing.events]
            sys_enter = true
        "#;

        let config = parse(toml_str);
        assert!(config.is_ok());

        if let Ok(c) = config {
            assert_eq!(c.log_level, "info".to_owned());
            assert_eq!(c.interval, 10);
            assert_eq!(c.metrics_host, "localhost".to_owned());
            assert_eq!(c.metrics_port, 9090);
            assert!(c.tracing.events.sys_enter);
        }
    }
}
