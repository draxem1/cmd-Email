pub mod text{
use std::process::Command;
use std::io;
use std::thread;
use std::time::Duration;

	pub enum Options<'a> {
		Done(&'a str),
		Clear(&'a str),
	}

	impl <'a>Options<'a> {
		pub fn choice(&'a self) -> &'a str{
			match self {
				Options::Done(n) => n,
				Options::Clear(n) => n, 
			}
		}
	}

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
		thread::sleep(Duration::from_millis(1000));
	}

	pub fn read_line() -> String{
	    let mut line = String::new();

	    io::stdin()
	        .read_line(&mut line)
	        .expect("Couldn't read line!!");

	    line
	}

pub fn help_menu(x: u32) {

match x{
		1 => {
let raw = 
r#"*******************************************
* Options -> "$F" = final, "$CL" = clear, *
*******************************************"#;
		println!("{}\n\n",raw);
	},
		2 => {
let raw = 
r#"*********************************************
* Options :         "-n" = new draft,       *
*                   "-r" = get saved draft  *
*                   "-l" = contact list     *
*                   "-h" = help menu        *
*                   "-ul" = new contact     * 
*********************************************"#;
		println!("{}\n\n",raw);
	},
	_ => (),
}

}}