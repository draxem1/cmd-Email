/**************************/
use std::process::Command;
use std::io;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Draft<T>{
	pub sender: T,
	pub reciever: T,
	subject: T,
	message: T,
}

pub trait Default{
	fn default() -> Self;
}

pub trait New{
	type Input;

	fn new(&self, _: Self::Input, 
				_: Self::Input,
				_: Self::Input, 
				_: Self::Input) -> Self;
}

pub trait Getter<T> {
	fn get_sender(&self) -> T;
	fn get_reciever(&self) -> T;
}

//Sets Generic T as lifetime &str
impl<'a> Default for Draft<&str>{
	fn default() -> Self{
		Self {
			sender: "",
			reciever: "",
			subject: "",
			message: "",
		}
	}
} 

impl <T>Getter<T> for Draft<T> 
	where T: Copy{
	fn get_sender(&self) -> T {
		self.sender
	}

	fn get_reciever(&self) -> T {
		self.reciever
	}
}

impl <T>New for Draft<T> {
	type Input = T;

	fn new(&self, sender: Self::Input, 
					reciever: Self::Input,
					subject: Self::Input,
					message: Self::Input) -> Self{
		Self {
			sender,
			reciever,
			subject,
			message,
		}
	}
}

//Because T become &str lifetime parameter takes place of T
//Refactor into another file
pub fn set_flag<'a, U>(run: &str, email: &U)
	where U: Getter<&'a str>{
		match run {
			"-n" => new_draft(&email.get_sender(), &email.get_reciever()),
			//"-r" => retrieve_saved_draft(),
				_=> (),
		}
}

//Can be a caller func in another file
pub fn read_line() -> String{
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Couldn't read line!!");

    line
}


//Refactor into another file
//needs seperate func with editing options of message body
pub fn new_draft(sender: &str, reciever: &str){
	let call = || -> String{
		read_line()
	};
	let draft = Draft::default();
	clear_screen();

	println!("Subject: ");
	let sub = String::from(call());
	let body = edited_message(call);
	let draft = draft.new(sender, reciever, &sub, &body);

	println!("\n\n{:?}", draft);
}

/*******************************
 * --CL: erase "message"
 * --D: Done
 * 
 * **************************/
 #[allow(unused_variables)]
pub fn edited_message<F>(call: F) -> String
	where F: Fn() -> String{
	let mut body = String::new();

	println!("\nMESSAGE:");

	while !body.contains("--D") {
		if body.contains("--CL") {
            body.clear();

		} else {
			body.push_str(&call());
		}
	}
	body.trim().to_string()
}

/*****
pub fn back_a_line(message: &str) -> String{
	let s = String::from(message);
	let mut m: Vec<&str> = s.split("\r").collect();
	let mut string = String::new();

	m.pop();

	for i in m {
		string.push_str(i);
	}

	string
}
****/

//Used threads to give next print macro time
pub fn clear_screen(){
	let handle = thread::spawn(|| {
		Command::new("cmd")
			.args(["/C","cls"])
			.spawn()
			.expect("Couldn't clear!!");
		thread::sleep(Duration::from_millis(1));
	});

	handle.join().unwrap();
	thread::sleep(Duration::from_millis(1));
}