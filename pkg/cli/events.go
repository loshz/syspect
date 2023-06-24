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

func NewEventsCommand(args []string) RunFunc {
	fs := flag.NewFlagSet(CommandEvents, flag.ExitOnError)

	fs.Usage = func() {
		fmt.Fprintln(os.Stderr, eventsUsage)
	}

	var verbose bool
	fs.BoolVar(&verbose, "v", false, "")
	fs.BoolVar(&verbose, "verbose", false, "")
	_ = fs.Parse(args)

	return events(verbose)
}

func events(verbose bool) RunFunc {
	return func() error {
		fmt.Printf("Verbose: %v\n", verbose)
		return nil
	}
}
