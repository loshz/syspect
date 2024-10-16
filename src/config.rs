use std::fs;

use serde::Deserialize;

use crate::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    /// The level at which logs should be omitted.
    pub log_level: String,

    /// Format logger for syslog.
    pub syslog: bool,

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
        let data = fs::read_to_string(path).map_err(Error::IO)?;
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

    // Test that parsing config with missing fields returns an error.
    #[test]
    fn test_parse_error() {
        let toml_str = r#"
            does_not_exist = true
        "#;

        let err = Config::parse(toml_str).unwrap_err();
        assert!(matches!(err, Error::Config(_)));
    }

    // Test that parsing a valid config file does not return an error.
    #[test]
    fn test_parse_success() {
        let toml_str = r#"
            log_level = "info"
            syslog = true
            metrics_addr = "localhost:9090"

            [tracing]
            interval = 10
            events = ["sys_enter"]
        "#;

        let config = Config::parse(toml_str).unwrap();
        assert_eq!(&config.log_level, "info");
        assert!(config.syslog);
        assert_eq!(&config.metrics_addr, "localhost:9090");
        assert_eq!(config.tracing.interval, 10);
        assert_eq!(&config.tracing.events, &["sys_enter"]);
    }
}
