package cli

import (
	"flag"
	"fmt"
	"os"
)

const startUsage = `Start the daemon and expose a local metrics HTTP endpoint

USAGE:
  syspect start [OPTIONS]

OPTIONS:
  -c, --config <PATH>   Path to the config file startation location [default: /etc/syspect.conf]
  -h, --help            Print help information`

func NewStartCommand() *Command {
	cmd := &Command{
		flags:   flag.NewFlagSet("start", flag.ExitOnError),
		Execute: start,
	}

	cmd.flags.Usage = func() {
		fmt.Fprintln(os.Stderr, startUsage)
	}

	return cmd
}

func start(cmd *Command, args []string) error {
	fmt.Println("start")
	return nil
}
