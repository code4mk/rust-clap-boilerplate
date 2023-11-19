mod my_commands;
mod process;
use clap::command;

fn main(){
    let match_result = command!()
        .about("this is dummy cli")
        .subcommand(my_commands::register_person::build_command())
        .get_matches();

    process::the_process::process_commands(&match_result)
    
}