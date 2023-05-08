package cli

import (
	"flag"
	"fmt"
	"os"
)

const eventsUsage = `List currently available Kernel trace events

USAGE:
  syspect events [OPTIONS]

OPTIONS:
  -v, --verbose  Whether to print the output verbosely
  -h, --help     Print help information`

func NewEventsCommand() *Command {
	cmd := &Command{
		flags:   flag.NewFlagSet("events", flag.ExitOnError),
		Execute: events,
	}

	cmd.flags.Usage = func() {
		fmt.Fprintln(os.Stderr, eventsUsage)
	}

	return cmd
}

func events(cmd *Command, args []string) error {
	fmt.Println("events")
	return nil
}
