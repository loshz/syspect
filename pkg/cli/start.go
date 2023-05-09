package cli

import (
	"flag"
	"fmt"
	"os"

	"github.com/loshz/syspect/pkg/config"
)

const (
	CommandStart = "start"
	startUsage   = `Start the daemon and expose a local metrics HTTP endpoint

USAGE:
  syspect start [OPTIONS]

OPTIONS:
  -c, --config <PATH>   Path to the config file startation location [default: /etc/syspect.conf]
  -h, --help            Print help information`
)

func NewStartCommand() *Command {
	fs := flag.NewFlagSet(CommandStart, flag.ExitOnError)

	var cfg string
	fs.StringVar(&cfg, "c", config.DefaultConfig, "")
	fs.StringVar(&cfg, "config", config.DefaultConfig, "")

	fs.Usage = func() {
		fmt.Fprintln(os.Stderr, startUsage)
	}

	return &Command{
		flags:   fs,
		Execute: start(&cfg),
	}
}

func start(cfg *string) ExecuteFunc {
	return func(cmd *Command, args []string) error {
		fmt.Printf("Config: %v\n", cfg)
		return nil
	}
}
