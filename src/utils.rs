use crate::structs::config::Config;

pub fn parse_command(command: String, config: &mut Config) {
    let command = command
        .split(" ")
        .collect::<Vec<&str>>();

    let key = command[0];
    let value = command[1];
    config[key] = value.parse().unwrap();
}
