use clap::{crate_version, Arg, ColorChoice, Command};

pub fn create_app() -> Command<'static> {
    let clap_color_choice = if std::env::var_os("NO_COLOR").is_none() {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    };

    let app = Command::new("appwash-cli")
        .name("AppWash CLI")
        .version(crate_version!())
        .author("olem")
        .about("AppWash CLI")
        .subcommand(
            Command::new("login")
                .about("Create account")
                .arg(
                    Arg::new("email")
                        .short('e')
                        .long("email")
                        .help("Your AppWash email"),
                )
                .arg(
                    Arg::new("password")
                        .short('p')
                        .long("password")
                        .help("Your AppWash password"),
                ),
        )
        .subcommand(Command::new("list").about("Lists available machines"))
        .subcommand(Command::new("reserve").about("Reserves a machine"));

    app
}
