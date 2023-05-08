package cli

import (
	"flag"
	"fmt"
	"os"
)

const uninstallUsage = `Remove config and systemd service files

USAGE:
  syspect uninstall [OPTIONS]

OPTIONS:
  -c, --config <PATH>   Path to the config file installation location [default: /etc/syspect.conf]
  -s, --service <PATH>  Path to the systemd service file installation location [default: /usr/lib/systemd/system/syspect.service]
  -h, --help            Print help information`

func NewUninstallCommand() *Command {
	cmd := &Command{
		flags:   flag.NewFlagSet("uninstall", flag.ExitOnError),
		Execute: uninstall,
	}

	cmd.flags.Usage = func() {
		fmt.Fprintln(os.Stderr, uninstallUsage)
	}

	return cmd
}

func uninstall(cmd *Command, args []string) error {
	fmt.Println("uninstall")
	return nil
}
