# Linux eBPF Monitoring Daemon (/ˈliːmərs/)
[![Build Status](https://github.com/loshz/lemurs/workflows/CI/badge.svg)](https://github.com/loshz/lemurs/actions)

> _Note: this is currently a side project so I can experiment with different libs. It should **NOT** be used in a production environment._

A Linux service that monitors systems via eBPF and exports data to Prometheus.

## BPF
The current BTF definitions were generated using the following Kernel:
```
$ uname -rvmo
5.16.16-arch1-1 #1 SMP PREEMPT Mon, 21 Mar 2022 22:59:40 +0000 x86_64 GNU/Linux
```
