package config

const (
	DefaultConfigPath = "/etc/syspect.conf"
)

var DefaultConfig = Config{}

// TODO: create default config.
type Config struct{}

// TODO: construct string from default config.
func (c *Config) String() string {
	return `# The log level of the service. On of info, debug, or error.
log_level = "info"
# The time in seconds between running each ebpf program. 
interval = 10
# The address of the host to bind the metrics server to.
metrics_host = "localhost"
# The port of the host to bind the metrics server to.
metrics_port = 9091

# Enable or disable specific eBPF tracepoints.
[tracing]

# Trace events found in /sys/kernel/debug/tracing/events
[tracing.events]
sys_enter = true`
}
