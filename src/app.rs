use clap::{arg, command, ArgAction, ColorChoice, Command};

pub fn get_app() -> Command {
    command!()
        .color(ColorChoice::Always)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("reserve").about("Reserves a machine").arg(
                arg!(<id> "The ID of the machine you want to stop")
                    .value_parser(clap::value_parser!(usize)),
            ),
        )
        .subcommand(
            Command::new("stop").about("Stops a machine").arg(
                arg!(<id> "The ID of the machine you want to stop")
                    .value_parser(clap::value_parser!(usize)),
            ),
        )
        .subcommand(Command::new("balance").about("Prints users balance"))
        .subcommand(
            Command::new("me")
                .about("Prints information about the user")
                .arg(arg!(-s --secrets "Show password and token").action(ArgAction::SetTrue)),
        )
        .subcommand(
            Command::new("list")
                .about("Lists available machines")
                .args([
                    arg!(-a --available "List available machines").action(ArgAction::SetTrue),
                    arg!(-o --occupied "List occupied machines").action(ArgAction::SetTrue),
                    arg!(-s --stoppable "List stoppable machines").action(ArgAction::SetTrue),
                ]),
        )
        .subcommand(Command::new("location").about("Get information about your location"))
        .subcommand(Command::new("history").about("Lists your activity and history"))
}
