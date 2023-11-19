use clap::{command, Arg};

fn main(){
    let match_result = command!()
        .about("this is dummy cli")
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
        .arg(
            Arg::new("fluffy")
                .long("fluffy")
        )
        .get_matches();
}