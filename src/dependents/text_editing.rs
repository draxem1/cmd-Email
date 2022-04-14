pub mod text{
use std::process::Command;
use std::io;
use std::thread;
use std::time::Duration;

	pub fn back_a_line(message: &str) -> String{
		let mut chunks: Vec<&str> = message.split("\r").collect();
		let mut string = String::new();

		chunks.pop();

		for i in chunks {
			let tmp = format!("\r{}", i);
			string.push_str(&tmp);
		}

		//println! here to give time for other print macros
		//after clear_screen is called
		println!("");
		string
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

}