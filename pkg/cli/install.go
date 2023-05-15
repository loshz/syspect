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
  -c, --config <PATH>   Path to the config file installation location
  -s, --service <PATH>  Path to the systemd service file installation location
  -h, --help            Print help information`
)

func NewInstallCommand(args []string) RunFunc {
	fs := flag.NewFlagSet(CommandInstall, flag.ExitOnError)

	fs.Usage = func() {
		fmt.Fprintln(os.Stderr, installUsage)
	}

	var cfg, svc string
	fs.StringVar(&cfg, "c", config.DefaultConfigPath, "")
	fs.StringVar(&cfg, "config", config.DefaultConfigPath, "")
	fs.StringVar(&svc, "s", config.DefaultSystemdPath, "")
	fs.StringVar(&svc, "service", config.DefaultSystemdPath, "")
	_ = fs.Parse(args)

	return install(cfg, svc)
}

func install(cfg, svc string) RunFunc {
	return func() error {
		// Write default config.
		if err := writeFile(cfg, config.DefaultConfig.String()); err != nil {
			return fmt.Errorf("error writing systemd service file: %w", err)
		}
		fmt.Println("Default config saved to:", cfg)

		// Write systemd service file.
		if err := writeFile(svc, config.DefaultSystemd); err != nil {
			return fmt.Errorf("error writing systemd service file: %w", err)
		}
		fmt.Println("systemd service saved to:", svc)

		return nil
	}
}

func writeFile(path, data string) error {
	f, err := os.Create(path)
	if err != nil {
		return err
	}
	defer f.Close()

	_, err = f.WriteString(data)
	return err
}
