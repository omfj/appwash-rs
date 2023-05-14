# CLI for AppWash written in Rust

> **Warning**
> I AM NOT AFFILIATED WITH MIELE OR APPWASH IN ANY WAY. USE THIS SOFTWARE AT YOUR OWN RISK. I AM NOT RESPONSIBLE FOR ANY DAMAGE OR LOSS CAUSED BY THIS SOFTWARE.

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
$ appwash

A command-line interface for Miele appWash

Usage: appwash <COMMAND>

Commands:
  reserve   Reserves a machine
  stop      Stops a machine
  balance   Prints users balance
  me        Prints information about the user
  list      Lists available machines
  location  Get information about your location
  history   Lists your activity and history
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## Remove/Uninstall

To remove the binary from your system run `make clean` when inside of the folder you installed from.

## TODO

- Better error handling
- Manage timezones better
- Store token between command usage

Rust is hard, please give me feedback.
