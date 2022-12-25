use clap::{
    crate_authors, crate_description, crate_name, crate_version, Arg, ColorChoice, Command,
};

pub fn create_app() -> Command {
    Command::new("appwash-cli")
        .name(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .color(ColorChoice::Always)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("reserve")
                .about("Reserves a machine")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("id")
                        .required(true)
                        .help("ID of the machine you want to reserve"),
                ),
        )
        .subcommand(
            Command::new("stop")
                .about("Stops a machine")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("id")
                        .required(true)
                        .help("ID of the machine you want to stop"),
                ),
        )
        .subcommand(Command::new("balance").about("Prints users balance"))
        .subcommand(
            Command::new("whoami")
                .about("Prints information about the user")
                .arg(
                    Arg::new("secrets")
                        .short('s')
                        .long("secrets")
                        .help("Prints you password and token"),
                ),
        )
        .subcommand(
            Command::new("update")
                .subcommand(
                    Command::new("email")
                        .about("Updates your config email")
                        .arg(Arg::new("email").required(true).help("Your new email")),
                )
                .subcommand(
                    Command::new("password")
                        .about("Updates your config password")
                        .arg(
                            Arg::new("password")
                                .required(true)
                                .help("Your new password"),
                        ),
                )
                .subcommand(
                    Command::new("location")
                        .about("Updates your config location")
                        .arg(
                            Arg::new("location")
                                .required(true)
                                .help("Your new location"),
                        ),
                )
                .about("Updates the specified field in your config"),
        )
        .subcommand(Command::new("location").about("Get information about your location"))
        .subcommand(Command::new("list").about("Lists available machines"))
        .subcommand(Command::new("history").about("Lists your activity and history"))
}
