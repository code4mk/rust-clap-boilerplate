use clap::{command, Arg, Command};

fn main(){
    let match_result = command!()
        .about("this is dummy cli")
        .subcommand(
            Command::new("register-person")
                .about("register a user")
                .arg(
                    Arg::new("firstname")
                        .short('f')
                        .long("first-name")
                        .alias("fname")
                        .required(true)
                        .help("user first name")
                )
                .arg(
                    Arg::new("lastname")
                        .short('l')
                        .long("last-name")
                        .required(true)
                        .help("user last name")
                )
        )
        .get_matches();

    // Use subcommand_matches to get matches for the specific subcommand
    if let Some(register_person_matches) = match_result.subcommand_matches("register-person") {
        // Extract values of the arguments
        let firstname = register_person_matches.get_one::<String>("firstname").unwrap();
        let lastname = register_person_matches.get_one::<String>("lastname").unwrap();
        // Perform actions with the provided values
        println!("Registering person: {} {}", firstname, lastname);
    }
}