use std::fs;

use anyhow::{Context, Error};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub log_level: Option<String>,
    pub prometheus: Prometheus,
}

#[derive(Debug, Deserialize)]
pub struct Prometheus {
    pub address: String,
    pub port: u16,
}

pub fn from_file(path: &str) -> Result<Config, Error> {
    let data = fs::read_to_string(path).unwrap();
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
            log_level = "info"
        "#;

        let config = parse(toml_str);
        assert!(config.is_err());
    }

    // Test that parsing a valid config file does not return an error.
    #[test]
    fn test_parse_success() {
        let toml_str = r#"
            log_level = "info"

            [prometheus]
            address = "localhost"
            port = 9090
        "#;

        let config = parse(toml_str);
        assert!(!config.is_err());

        if let Ok(c) = config {
            assert_eq!(c.log_level, Some("info".to_owned()));
            assert_eq!(c.prometheus.address, "localhost".to_owned());
            assert_eq!(c.prometheus.port, 9090);
        }
    }
}
