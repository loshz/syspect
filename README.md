# syspect - Linux eBPF Monitoring Daemon
> **Note**: this is currently a side project so I can experiment with different libs. It should **NOT** be used in a production environment.

[![Build Status](https://github.com/loshz/syspect/workflows/ci/badge.svg)](https://github.com/loshz/syspect/actions) [![GPL-3.0 licensed](https://img.shields.io/badge/license-GPL--3.0-blue)](LICENSE)

A Linux service that monitors systems via eBPF and exports data to Prometheus.

## Usage
```
$ sudo syspect --help
syspect 0.1.0
Linux eBPF Monitoring daemon

USAGE:
    syspect <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    install      Install default config and systemd service files
    start        Start the daemon and expose a local metrics HTTP endpoint
    probes       List currently supported eBPF probes
    uninstall    Remove config and systemd service files
    help         Print this message or the help of the given subcommand(s)
```

### Config
The default config file is located at `/etc/syspect.conf`. A detailed example can be found [here](./config/syspect.conf).

## BPF
TODO: document included bpf probes.
