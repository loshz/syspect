package cli

import (
	"flag"
	"fmt"
	"os"
)

const (
	CommandEvents = "events"
	eventsUsage   = `List currently available Kernel trace events

USAGE:
  syspect events [OPTIONS]

OPTIONS:
  -v, --verbose  Whether to print the output verbosely
  -h, --help     Print help information`
)

func NewEventsCommand() *Command {
	fs := flag.NewFlagSet(CommandEvents, flag.ExitOnError)

	var verbose bool
	fs.BoolVar(&verbose, "v", false, "")
	fs.BoolVar(&verbose, "verbose", false, "")

	fs.Usage = func() {
		fmt.Fprintln(os.Stderr, eventsUsage)
	}

	return &Command{
		flags:   fs,
		Execute: events(&verbose),
	}
}

func events(verbose *bool) ExecuteFunc {
	return func(cmd *Command, args []string) error {
		fmt.Printf("Verbose: %v\n", verbose)
		return nil
	}
}
