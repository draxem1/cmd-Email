//send email messages from the command line
//cargo run -- address option

use cli_email::Default;
use std::env;
use std::process;
use std::result::Result;
//use std::process::Command; // runs os commands
use cli_email::{Draft, help_menu};

fn main() {
    let input = all_args_exist().unwrap_or_else(|err| {
        help_menu(2);
        eprintln!("Err: {}", err);
        process::exit(0);
    });

    let mut draft = Draft::default();
    draft.sender = &input[1];

    if input.len() >3 {
        draft.reciever = &input[3];
    }

    let option = &input[2];
    cli_email::set_flag(option, &draft);
}

fn all_args_exist() -> Result<Vec<String>, &'static str> 
    {
        let input: Vec<String> = env::args().collect();

        if input.len() < 3 {
            return Err("Not enough arguments");
        }

        Ok(input)
}
/*************
Command::new("cmd")
        .args(["/C" , "npm"])
        .spawn()
        .expect("ls command failed to start");
***********/