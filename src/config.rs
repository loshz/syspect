use std::fs;

use serde::Deserialize;

use crate::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    /// The host:port address to serve metrcis on.
    pub metrics_addr: String,

    /// Tracing specific config.
    pub tracing: Tracing,
}

#[derive(Debug, Deserialize)]
pub struct Tracing {
    /// The interval at which bpf probes should be polled.
    pub interval: u64,

    /// List of enabled trace events.
    pub events: Vec<String>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Config, Error> {
        let data = fs::read_to_string(path).map_err(|e| Error::Config(e.to_string()))?;
        let config = Self::parse(data.as_str())?;
        Ok(config)
    }

    fn parse(data: &str) -> Result<Config, Error> {
        let config: Config = toml::from_str(data).map_err(|e| Error::Config(e.message().into()))?;

        // Check events have been registered.
        // TODO: use a default list?
        if config.tracing.events.is_empty() {
            return Err(Error::Config("no tracing events specified".into()));
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;

    use super::Config;

    #[test]
    fn test_parse_invalid_field_error() {
        let toml_str = r#"
            does_not_exist = true
        "#;

        let err = Config::parse(toml_str).unwrap_err();
        assert!(matches!(err, Error::Config(_)));
    }

    #[test]
    fn test_parse_empty_events_error() {
        let toml_str = r#"
            metrics_addr = "localhost:9090"

            [tracing]
            interval = 10
            events = []
        "#;

        let err = Config::parse(toml_str).unwrap_err();
        assert!(matches!(err, Error::Config(_)));
    }

    #[test]
    fn test_parse_success() {
        let toml_str = r#"
            metrics_addr = "localhost:9090"

            [tracing]
            interval = 10
            events = ["sys_enter"]
        "#;

        let config = Config::parse(toml_str).unwrap();
        assert_eq!(&config.metrics_addr, "localhost:9090");
        assert_eq!(config.tracing.interval, 10);
        assert_eq!(&config.tracing.events, &["sys_enter"]);
    }
}
