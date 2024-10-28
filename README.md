# syspect - Linux eBPF Monitoring Daemon
[![builds.sr.ht status](https://builds.sr.ht/~loshz/syspect.svg)](https://builds.sr.ht/~loshz/syspect?) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue)](LICENSE) [![GPL-3.0 licensed](https://img.shields.io/badge/license-GPL--3.0-blue)](bpf/LICENSE)

A Linux service that monitors systems via eBPF and exports data to Prometheus.

## Usage
```
$ syspect --help
Linux eBPF Monitoring daemon

Usage: syspect <COMMAND>

Commands:
  install    Install default config and systemd service files
  start      Start the daemon and expose a local metrics HTTP endpoint
  events     List currently available Kernel trace events
  uninstall  Remove config and systemd service files

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Config
The default config file is located at `/etc/syspect.conf`. A detailed example can be found [here](./config/syspect.conf).

### Docker
If you'd like to quickly get monitoring up and running, default Grafana and Prometheus configs have been included and can be ran using `docker compose up` from the project root dir. It assumes `syspect` is running on port 9091.

## BPF
TODO: document included bpf probes.
