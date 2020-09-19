## This has been moved into the [cx](https://github.com/tmtmtoo/cx).

# :eyes: rty

a simple command line retry tool.

```
rty
Retry command execution until successful.

USAGE:
    rty [OPTIONS] [COMMAND]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --count <count>          maximum number of executions
    -i, --interval <interval>    execution interval (sec) [default: 0.1]

ARGS:
    <COMMAND>...    command and options
```

## example

```bash
$ rty -c 2 -i 2 -- your command that may fail && echo succeeded || echo failed
```
