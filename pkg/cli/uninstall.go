package cli

import (
	"flag"
	"fmt"
	"os"

	"github.com/loshz/syspect/pkg/config"
)

const (
	CommandUninstall = "uninstall"
	uninstallUsage   = `Remove config and systemd service files

USAGE:
  syspect uninstall [OPTIONS]

OPTIONS:
  -c, --config <PATH>   Path to the config file installation location
  -s, --service <PATH>  Path to the systemd service file installation location
  -h, --help            Print help information`
)

func NewUninstallCommand(args []string) RunFunc {
	fs := flag.NewFlagSet(CommandUninstall, flag.ExitOnError)

	fs.Usage = func() {
		fmt.Fprintln(os.Stderr, uninstallUsage)
	}

	var cfg, svc string
	fs.StringVar(&cfg, "c", config.DefaultConfigPath, "")
	fs.StringVar(&cfg, "config", config.DefaultConfigPath, "")
	fs.StringVar(&svc, "s", config.DefaultSystemdPath, "")
	fs.StringVar(&svc, "service", config.DefaultSystemdPath, "")
	_ = fs.Parse(args)

	return uninstall(cfg, svc)
}

func uninstall(cfg, svc string) RunFunc {
	return func() error {
		if err := os.Remove(cfg); err != nil {
			return fmt.Errorf("error removing config: %w", err)
		}
		fmt.Println("Removed config file:", cfg)

		if err := os.Remove(svc); err != nil {
			return fmt.Errorf("error removing systemd service file: %w", err)
		}
		fmt.Println("Removed systemd service file:", svc)

		return nil
	}
}
