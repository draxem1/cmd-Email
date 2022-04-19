/**************************/
#[path = "./dependents/text_editing.rs"]
mod text_editing;

#[path = "./dependents/database.rs"]
mod database;

#[path = "./dependents/email.rs"]
mod email;

use serde::{Serialize, Deserialize};
pub use crate::text_editing::text::*;
pub use database::data::*;
use email::sender::*;

#[derive(Debug, Deserialize, Serialize)]
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

pub trait Edit {
	fn edit(&self) -> (String, String);
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

impl <T>Edit for Draft<T>{
	fn edit(&self) -> (String, String){
		let done = Options::Done("$F");
		let erase = Options::Clear("$CL");

		let subject = || -> String{
			clear_screen();
			help_menu(1);

			println!("Subject:");
			let string = read_line();
			string
		};

		let message = || -> String{
			let mut string = String::new();

			'outer: loop {
				clear_screen();
				help_menu(1);
				println!("Message:");

					print!("{}",&string);

				'inner: loop {
					if string.contains(done.choice()) {
						break 'outer string;

					} else if string.contains(erase.choice()) {
						string.clear();
						break 'inner;

					} else {
						string.push_str(&read_line());
					}
				}
			}
		};

		(subject(),message())
	}
}

//Because T become &str lifetime parameter takes place of T
//Decision tree from input
pub fn set_flag<'a, U>(run: &str, email: &U)
	where U: Getter<&'a str>{
		match run {
			"-n" => { let draft = Draft::default();
				let tuple = draft.edit(); 
				let new = draft.new(
					email.get_sender(),
					email.get_reciever(),
					&tuple.0.trim(),
					&tuple.1.trim(),
				); 
				new.full_send();
			},
			"-l" => contact_list(),
			"-ul" => new_contact(),
			"-rc" => remove_contact(),
			"-h" => help_menu(2),
				_=> {println!("Not a valid option");
					help_menu(2);},
		}
}