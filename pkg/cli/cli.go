package cli

import "flag"

type ExecuteFunc func(cmd *Command, args []string) error

type Command struct {
	flags   *flag.FlagSet
	Execute ExecuteFunc
}

func (c *Command) Init(args []string) {
	_ = c.flags.Parse(args)
}

func (c *Command) Run() error {
	return c.Execute(c, c.flags.Args())
}
