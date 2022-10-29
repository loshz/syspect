# syspect - Linux eBPF Monitoring Daemon
> **Note**: this is currently a side project so I can experiment with different libs. It should **NOT** be used in a production environment.

[![Build Status](https://github.com/loshz/syspect/workflows/ci/badge.svg)](https://github.com/loshz/syspect/actions) [![GPL-3.0 licensed](https://img.shields.io/badge/license-GPL--3.0-blue)](LICENSE)

A Linux service that monitors systems via eBPF and exports data to Prometheus.

## Usage
```
$ sudo syspect --help
Linux eBPF Monitoring daemon

Usage: syspect <COMMAND>

Commands:
  install    Install default config and systemd service files
  start      Start the daemon and expose a local metrics HTTP endpoint
  events     List currently available Kernel trace events
  uninstall  Remove config and systemd service files
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

### Config
The default config file is located at `/etc/syspect.conf`. A detailed example can be found [here](./config/syspect.conf).

### Docker
If you'd like to quickly get monitoring up and running, default Grafana and Prometheus configs have been included and can be ran using `docker compose up` from the project root dir. It assumes `syspect` is running on port 9091.

## BPF
TODO: document included bpf probes.
