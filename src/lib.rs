/**************************/
use std::io;

pub struct Draft<T>{
	pub sender: T,
	pub reciever: T,
	subject: T,
	message: T,
}

pub trait Default{
	fn default() -> Self;
}

pub trait Getter<T> {
	fn get_sender(&self) -> T;
	fn get_reciever(&self) -> T;
}

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

//do command from args and input
pub fn set_flag<U, T>(run: &str, email: &U)
	where U: Getter<T>{
		match run {
			"-n" => new_draft(&email.get_sender(), &email.get_reciever()),
				_=> (),
		}
}

pub fn read_line() -> String{
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Couldn't read line!!");

    line
}


pub fn new_draft<T>(_sender: T, _reciever: T){

	let mut _draft = Draft::default();

	println!("Subject: ");
	let _sub = String::from(read_line().trim());

	println!("Message: ");
	let mut body = String::new();

	while !body.contains("-S"){
		if body.contains("-CL") {
			body.clear();
		} else {
			body.push_str(read_line().trim());
		}
	}
}