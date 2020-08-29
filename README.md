# :eyes: rty

a simple command line retry tool.

```
Usage: rty <command> [-c <count>] [-i <interval>]

Retry command execution until successful.

Options:
  -c, --count       maximum number of executions
  -i, --interval    execution interval (sec)
  --help            display usage information
```

## example

```sh
$ rty "your command that may fail" -c 2 -i 2 && echo succeeded || echo failed
```
