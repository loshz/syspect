package cli

import (
	"flag"
	"fmt"
	"os"

	"github.com/loshz/syspect/pkg/config"
)

const (
	CommandInstall = "install"
	installUsage   = `Install default config and systemd service files

USAGE:
  syspect install [OPTIONS]

OPTIONS:
  -c, --config <PATH>   Path to the config file installation location [default: /etc/syspect.conf]
  -s, --service <PATH>  Path to the systemd service file installation location [default: /usr/lib/systemd/system/syspect.service]
  -h, --help            Print help information`
)

func NewInstallCommand() *Command {
	fs := flag.NewFlagSet(CommandInstall, flag.ExitOnError)

	var cfg, svc string
	fs.StringVar(&cfg, "c", config.DefaultConfig, "")
	fs.StringVar(&cfg, "config", config.DefaultConfig, "")
	fs.StringVar(&svc, "s", config.DefaultSystemd, "")
	fs.StringVar(&svc, "service", config.DefaultSystemd, "")

	fs.Usage = func() {
		fmt.Fprintln(os.Stderr, installUsage)
	}

	return &Command{
		flags:   fs,
		Execute: install(&cfg, &svc),
	}
}

func install(cfg, svc *string) ExecuteFunc {
	return func(cmd *Command, args []string) error {
		fmt.Printf("Config: %v\n", cfg)
		fmt.Printf("Service: %v\n", svc)
		return nil
	}
}
