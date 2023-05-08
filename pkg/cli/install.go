package cli

import (
	"flag"
	"fmt"
	"os"
)

const installUsage = `Install default config and systemd service files

USAGE:
  syspect install [OPTIONS]

OPTIONS:
  -c, --config <PATH>   Path to the config file installation location [default: /etc/syspect.conf]
  -s, --service <PATH>  Path to the systemd service file installation location [default: /usr/lib/systemd/system/syspect.service]
  -h, --help            Print help information`

func NewInstallCommand() *Command {
	cmd := &Command{
		flags:   flag.NewFlagSet("install", flag.ExitOnError),
		Execute: install,
	}

	cmd.flags.Usage = func() {
		fmt.Fprintln(os.Stderr, installUsage)
	}

	return cmd
}

func install(cmd *Command, args []string) error {
	fmt.Println("install")
	return nil
}
