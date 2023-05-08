package cli

const Usage = `Linux Monitoring Daemon

USAGE:
  syspect <COMMAND>

COMMANDS:
  install        Install default config and systemd service files
  start          Start the daemon and expose a local metrics HTTP endpoint
  events         List currently available Kernel trace events
  uninstall      Remove config and systemd service files

OPTIONS:
  -h, --help     Print command-specific usage
  -V, --version  Print version information
`
