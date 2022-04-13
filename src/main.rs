//send email messages from the command line
//cargo run -- address option
use cli_email::Default;
use std::env;
//use std::process::Command; // runs os commands
use cli_email::Draft;

fn main() {
    let input: Vec<String> = env::args().collect();

    let mut draft = Draft::default();
    draft.sender = &input[1];
    draft.reciever = &input[2];
    let option = &input[3];

    cli_email::set_flag(option, &draft);
}

/*************
Command::new("cmd")
        .args(["/C" , "npm"])
        .spawn()
        .expect("ls command failed to start");
***********/