package main

import (
	"fmt"
	"os"

	"github.com/loshz/syspect/pkg/cli"
	"github.com/loshz/syspect/pkg/version"
)

func main() {
	if len(os.Args) < 2 {
		usage()
		os.Exit(2)
	}

	var run cli.RunFunc

	switch os.Args[1] {
	case cli.CommandInstall:
		run = cli.NewInstallCommand(os.Args[2:])
	case cli.CommandStart:
		run = cli.NewStartCommand(os.Args[2:])
	case cli.CommandEvents:
		run = cli.NewEventsCommand(os.Args[2:])
	case cli.CommandUninstall:
		run = cli.NewUninstallCommand(os.Args[2:])
	case "-V", "--version":
		fmt.Printf("syspect %s\n", version.Version)
		os.Exit(0)
	case "-h", "--help":
		usage()
		os.Exit(0)
	default:
		usage()
		os.Exit(2)
	}

	// Check if root.
	if os.Geteuid() != 0 {
		fmt.Fprintln(os.Stderr, "not running as root")
		//os.Exit(1)
	}

	if err := run(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func usage() {
	fmt.Printf("syspect %s\n", version.Version)
	fmt.Println(cli.Usage)
}
