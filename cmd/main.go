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
	}

	var cmd *cli.Command

	switch os.Args[1] {
	case cli.CommandInstall:
		cmd = cli.NewInstallCommand()
	case cli.CommandStart:
		cmd = cli.NewStartCommand()
	case cli.CommandEvents:
		cmd = cli.NewEventsCommand()
	case cli.CommandUninstall:
		cmd = cli.NewUninstallCommand()
	case "-V", "--version":
		fmt.Printf("syspect %s\n", version.Version)
		os.Exit(0)
	default:
		usage()
	}

	cmd.Init(os.Args[2:])
	if err := cmd.Run(); err != nil {
		os.Exit(1)
	}
}

func usage() {
	fmt.Printf("syspect %s\n", version.Version)
	fmt.Println(cli.Usage)
	os.Exit(1)
}
