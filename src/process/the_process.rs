use clap::ArgMatches;
use crate::my_commands::register_person::process_command as register_user;

pub fn process_commands(match_result: &ArgMatches) {
  register_user(match_result)
}