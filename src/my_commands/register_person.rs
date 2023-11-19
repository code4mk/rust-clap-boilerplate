use clap::{Arg, Command, ArgMatches};

pub fn build_command() -> Command {
    Command::new("register-person")
        .about("register a user")
        .arg(
            Arg::new("firstname")
                .short('f')
                .long("first-name")
                .alias("fname")
                .required(true)
                .help("user first name"),
        )
        .arg(
            Arg::new("lastname")
                .short('l')
                .long("last-name")
                .required(true)
                .help("user last name"),
        )
}

pub fn process_command(match_result: &ArgMatches) {
    if let Some(register_person_matches) = match_result.subcommand_matches("register-person") {
      let firstname = register_person_matches.get_one::<String>("firstname").unwrap();
      let lastname = register_person_matches.get_one::<String>("lastname").unwrap();
      println!("Registering person: {} {}", firstname, lastname);
    }
}