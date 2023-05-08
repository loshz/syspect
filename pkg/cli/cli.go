package cli

import "flag"

type ExecuteFunc func(cmd *Command, args []string) error

type Command struct {
	flags   *flag.FlagSet
	Execute ExecuteFunc
}

func (c *Command) Init(args []string) error {
	return c.flags.Parse(args)
}

func (c *Command) Called() bool {
	return c.flags.Parsed()
}

func (c *Command) Run() error {
	return c.Execute(c, c.flags.Args())
}
