# Linux eBPF Monitoring Daemon (/ˈliːmərs/)
> _Note: this is currently a side project so I can experiment with different libs. It should **NOT** be used in a production environment._

[![Build Status](https://github.com/loshz/lemurs/workflows/ci/badge.svg)](https://github.com/loshz/lemurs/actions) [![GPL-3.0 licensed](https://img.shields.io/badge/license-GPL--3.0-blue)](LICENSE)

A Linux service that monitors systems via eBPF and exports data to Prometheus.

## BPF
The current BTF definitions were generated using the following Kernel:
```
$ uname -rvmo
5.16.16-arch1-1 #1 SMP PREEMPT Mon, 21 Mar 2022 22:59:40 +0000 x86_64 GNU/Linux
```
