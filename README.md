# CLI for AppWash written in Rust

CLI for the website [appwash.com](https://appwash.com/). Uses the same API.

This projects is going to be the same as [appwash-cli](https://github.com/omfj/appwash-cli), but written in Rust.

## Install

If you are on Mac and have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed you use the Makefile in the repo.

```bash
git clone git@github.com:omfj/appwash-rs
cd appwash-rs
```

```bash
make all
```

Now the binary should be installed to your system. You can type `appwash` into your terminal emulator and the output should look similar to this:

```bash
➜ appwash
appwash-cli 1.0.0
Ole Magnus Johnsen <hei@omfj.no>
A command-line interface for Miele appWash

USAGE:
    appwash <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    balance     Prints users balance
    help        Print this message or the help of the given subcommand(s)
    history     Lists your activity and history
    list        Lists available machines
    location    Get information about your location
    login       Create account
    reserve     Reserves a machine
    stop        Stops a machine
    whoami      Prints information about the user

```

## Remove/Uninstall

To remove the binary from your system run `make clean` when inside of the folder you installed from.

## TODO

- Fix command: reserve
- Fix command: stop
- Better error handling
- Manage timezones better
- Store token between command usage

Rust is hard, please give me feedback.
