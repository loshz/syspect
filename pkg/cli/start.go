package cli

import (
	"context"
	"flag"
	"fmt"
	"os"
	"os/signal"
	"syscall"

	"github.com/rs/zerolog/log"

	"github.com/loshz/syspect/pkg/config"
	"github.com/loshz/syspect/pkg/metrics"
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
		// TODO: load config.
		log.Info().Msgf("using config: %s", cfg)

		// Listen for stop signal.
		stop := make(chan os.Signal, 1)
		signal.Notify(stop, syscall.SIGINT, syscall.SIGTERM)

		// Listen for service errors.
		errCh := make(chan error)

		// Start metrics server.
		srv := metrics.NewServer(8081, errCh)

		// Wait for signal to be received.
		select {
		case <-stop:
			log.Info().Msg("stop signal received, starting shut down")
			_ = srv.Shutdown(context.Background())
		case err := <-errCh:
			return fmt.Errorf("service error: %w", err)
		}

		return nil
	}
}
