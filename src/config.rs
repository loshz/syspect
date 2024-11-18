use std::{fs, time::Duration};

use serde::{de::Deserializer, Deserialize};

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
    /// Enable debug output.
    pub debug: bool,

    /// The interval (seconds) at which bpf probes should be polled.
    #[serde(deserialize_with = "deserialize_duration_seconds")]
    pub interval: Duration,

    /// List of raw_syscall events: `/sys/kernel/debug/tracing/events/raw_syscalls`
    pub raw_syscalls: Vec<String>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Config, Error> {
        let data = fs::read_to_string(path).map_err(Error::IO)?;
        let config = Self::parse(data.as_str())?;
        Ok(config)
    }

    fn parse(data: &str) -> Result<Config, Error> {
        let config: Config = toml::from_str(data).map_err(|e| Error::Config(e.message().into()))?;

        if config.tracing.debug {
            println!("Debug output enabled");
        }

        // Check events have been registered.
        // TODO: use a default list?
        if config.tracing.raw_syscalls.is_empty() {
            return Err(Error::Config("no tracing events specified".into()));
        }

        Ok(config)
    }
}

fn deserialize_duration_seconds<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let secs = u64::deserialize(deserializer)?;
    Ok(Duration::from_secs(secs))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

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
            raw_syscalls = []
        "#;

        let err = Config::parse(toml_str).unwrap_err();
        assert!(matches!(err, Error::Config(_)));
    }

    #[test]
    fn test_parse_success() {
        let toml_str = r#"
            metrics_addr = "localhost:9090"

            [tracing]
            debug = true
            interval = 10
            raw_syscalls = ["sys_enter"]
        "#;

        let config = Config::parse(toml_str).unwrap();
        assert_eq!(&config.metrics_addr, "localhost:9090");
        assert!(&config.tracing.debug);
        assert_eq!(config.tracing.interval, Duration::from_secs(10));
        assert_eq!(&config.tracing.raw_syscalls, &["sys_enter"]);
    }
}
