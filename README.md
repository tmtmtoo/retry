# :eyes: supervise

a simple command line supervisor tool.

```
Usage: supervise <command> [-c <count>] [-i <interval>]

Supervise command execution.

Options:
  -c, --count       maximum number of executions
  -i, --interval    execution interval (sec)
  --help            display usage information
```

## usage
```sh
$ supervice "echo abc" -c 2 -i 2
```
