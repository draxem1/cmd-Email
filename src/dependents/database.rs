pub mod data {
use crate::clear_screen;
use crate::read_line;
use std::process;
use serde::{Serialize, Deserialize};
use std::fs;
	
	#[derive(Debug, Serialize, Deserialize)]
	struct List{
		list: Vec<Contact>,
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Contact{
		id: usize,
		email: String,
		first_name: String,
		last_name: String,
	}

/************Traits****************/
	trait Add{
		fn add_to_list(&mut self, contact: Contact);
	}

	trait Delete{
		fn del_from_list(&mut self, id: usize);
	}

	trait Default {
		fn default() -> List;
	}
	trait Pull {
		fn pull_from_file(&mut self);
	}

	trait Save {
		fn push_to_file(&self);
	} 

/*************Impl of Traits*************/
	impl Add for List{
		fn add_to_list(&mut self, mut contact: Contact) {
			let id = self.list.len();
			contact.id = id;
			self.list.push(contact);
			self.push_to_file();
		}
	} 

	impl Delete for List{
		fn del_from_list(&mut self, id: usize) {

			if id < self.list.len() {
				self.list.remove(id);
			} else {
				println!("Id not valid!!");
				process::exit(0);
			}
		}
	}

//data from File
	impl Pull for List {
		fn pull_from_file(&mut self) {
			let list = fs::read_to_string("list.txt")
							.expect("failed to open!");

			let mut count = 0;
			let deserialized;

			loop {
				if count == 1 {
					let l = List::default();
					l.push_to_file();
					self.pull_from_file();
				}

				deserialized = 
				match serde_json::from_str(&list) {
					Ok(l) => l,
					Err(_) => {count = 1; continue;},
				};
				break;
			}
			self.list = deserialized;
		}
	}

//data to File
	impl Save for List{
		fn push_to_file(&self) {
			let serialized = serde_json::to_string(&self.list).unwrap(); 

			fs::write("list_copy.txt", serialized)
				.expect("Failed to write to file");

			fs::copy("list_copy.txt", "list.txt").unwrap();
		}
	}

	impl Default for List{
			fn default() -> List{
			List {
				list: vec![ Contact {
					id: 0,
					email: String::new(),
					first_name: String::new(),
					last_name: String::new(),
				}],
			}
		}
	}

/***********Essential Functions****************/
	fn contact_info() -> Contact{
		println!("First Name?");
		let first = read_line();

		println!("Last Name?");
		let last = read_line();

		println!("Email Address?");
		let address = read_line();

		let contact = Contact {
			id: 0,
			email: address.trim().to_string(),
			first_name: first.trim().to_string(),
			last_name: last.trim().to_string(),
		};

		contact
	}

	pub fn contact_list() {
		let mut list = List::default();
		list.pull_from_file();

		clear_screen();

		println!("**********CONTACT LIST**********");
		for i in list.list.iter().skip(1){
			println!("ID: \t\t{}",i.id);
			println!("First Name: \t{}", i.first_name);
			println!("Last Name: \t{}", i.last_name);
			println!("Email: \t\t{}\n", i.email);
		}
	}

	pub fn new_contact() {
		let person = contact_info();
		let mut list = List::default();
		list.pull_from_file();

		list.add_to_list(person);

		clear_screen();

		println!("\nContact Created..");
	}

	pub fn remove_contact() {
		let mut list = List::default();
		list.pull_from_file();

		clear_screen();
		contact_list();

		println!("Enter Id of Contact...");
		let who: u32 = read_line()
						.trim()
						.parse()
						.expect("not a valid Id");
								

		list.del_from_list(who as usize);
		list.push_to_file();

		clear_screen();
		println!("\nContact Removed");
	}
}