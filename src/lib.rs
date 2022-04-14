/**************************/
#[path = "./dependents/text_editing.rs"]
mod text_editing;

use crate::text_editing::text::{back_a_line, clear_screen, read_line};

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


//Refactor into another file
//needs seperate func with editing options of message body
pub fn new_draft(sender: &str, reciever: &str){

	let call = || -> String{
		read_line()
	};

	let draft = Draft::default();
	let mut sub = String::new();
	let mut count = 0;
	let mut cnt2 = 0;
	let mut body = String::new();

	loop {
		clear_screen();
		println!("Subject:");

		if count == 0 {
			sub = call();
			count += 1;
		} else {
			print!("{}", &sub);
		}
		println!("\nMessage:");

		if cnt2 > 0 {
			print!("{}", &body);
		}
		while !body.contains("--D") {

			if body.contains("--CL") {
				body.clear();
				break;
			}
			else if body.contains("--B") {
				cnt2 += 1;
				let tmp = back_a_line(&body);
				body.clear();
				body.push_str(&tmp);
				break;

			} else {
				body.push_str(&call());
			}
		}

		if body.contains("--D") {
			break;
		}
	}

	let draft = draft.new(sender, reciever, &sub, &body.trim());
	println!("\n\n{:?}", draft);
}