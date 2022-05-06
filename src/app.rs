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
        .subcommand(Command::new("list").about("Lists available machines"))
        .subcommand(Command::new("reserve").about("Reserves a machine"))
        .subcommand(Command::new("whoami").about("Prints information about the user"));

    app
}
