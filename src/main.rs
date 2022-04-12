//send email messages from the command line
//cargo run -- address option
use std::env;
use std::io;
use std::process::Command; // runs os commands

#[allow(dead_code)]
#[derive(Debug)]
struct Draft<T> {
    address: T,
    subject: T,
    message: T,
}

fn main() {
    let options: Vec<String> = env::args().collect();

    let email = &options[1];
    let flags = &options[2];

    println!("email = {}, flag = {}", email, flags);


   Command::new("cmd")
        .args(["/C" , "npm"])
        .spawn()
        .expect("ls command failed to start");
}

fn _validate_email(){}

fn _read_line() -> String{
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Couldn't read line!!");

    line
}
