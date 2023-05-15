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
  -c, --config <PATH>   Path to the config file startation location
  -h, --help            Print help information`
)

func NewStartCommand(args []string) RunFunc {
	fs := flag.NewFlagSet(CommandStart, flag.ExitOnError)

	fs.Usage = func() {
		fmt.Fprintln(os.Stderr, startUsage)
	}

	var cfg string
	fs.StringVar(&cfg, "c", config.DefaultConfigPath, "")
	fs.StringVar(&cfg, "config", config.DefaultConfigPath, "")
	_ = fs.Parse(args)

	return start(cfg)
}

func start(cfg string) RunFunc {
	return func() error {
		fmt.Printf("Config: %s\n", cfg)
		return nil
	}
}
