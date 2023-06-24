package config

const (
	DefaultSystemdPath = "/usr/lib/systemd/system/syspect.service"
	DefaultSystemd     = `[Unit]
Type=simple
Description=Linux eBPF Monitoring daemon.
ConditionFileIsExecutable=/usr/bin/syspect

[Service]
ExecStart=/usr/bin/syspect start --syslog
TimeoutSec=10

[Install]
WantedBy=multi-user.target`
)
