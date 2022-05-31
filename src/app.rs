use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};

pub fn create_app() -> Command<'static> {
    let app = Command::new("appwash-cli")
        .name(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("login")
                .about("Create account")
                .arg(
                    Arg::new("email")
                        .short('e')
                        .long("email")
                        .takes_value(true)
                        .required(true)
                        .help("Your AppWash email"),
                )
                .arg(
                    Arg::new("password")
                        .short('p')
                        .long("password")
                        .takes_value(true)
                        .required(true)
                        .help("Your AppWash password"),
                ),
        )
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
                        .takes_value(false)
                        .help("Prints you password and token"),
                ),
        )
        .subcommand(
            Command::new("location")
                .about("Get information about your location")
                .subcommand(
                    Command::new("change")
                        .arg(Arg::new("location").required(true).help("Location ID"))
                        .about("Change to another location"),
                ),
        )
        .subcommand(Command::new("list").about("Lists available machines"))
        .subcommand(Command::new("history").about("Lists your activity and history"));

    app
}
