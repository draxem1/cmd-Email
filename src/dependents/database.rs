pub mod data {
use crate::clear_screen;
use crate::read_line;
use serde::{Serialize, Deserialize};
use std::fs;
	
	trait Add{
		fn add_to_list(&mut self, contact: Contact);
	}

	trait Delete{
		fn del_from_list(&mut self);
	}

	trait Default {
		fn default() -> List;
	}
	pub trait Pull {
		fn pull_from_file(&mut self);
	}

	pub trait Save {
		fn push_to_file(&self);
	} 

	#[derive(Debug, Serialize, Deserialize)]
	struct List{
		list: Vec<Contact>,
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Contact{
		email: String,
		first_name: String,
		last_name: String,
	}

	impl Add for List{
		fn add_to_list(&mut self, contact: Contact) {
			self.list.push(contact);
		}
	} 

	impl Delete for List{
		fn del_from_list(&mut self) {
			self.list.pop();
		}
	}

	impl Pull for List {
		fn pull_from_file(&mut self) {
			let list = fs::read_to_string("list.txt")
							.expect("failed to open!");

			let deserialized;
			loop {
				deserialized = 
				match serde_json::from_str(&list) {
					Ok(l) => l,
					Err(_) => {let l = List::default();
							l.push_to_file();
							continue;},
				};
				break;
			}
			self.list = deserialized;
		}
	}

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
					email: String::new(),
					first_name: String::new(),
					last_name: String::new(),
				}],
			}
		}
	}
	pub fn contact_list() {
		let mut list = List::default();
		list.pull_from_file();

		clear_screen();

		println!("**********CONTACT LIST**********");
		for i in list.list{
			println!("First Name: {}", i.first_name);
			println!("Last Name: {}", i.last_name);
			println!("Email: {}", i.email);
		}
		

	}

	pub fn get_contact() -> Contact{
		println!("First Name?");
		let first = read_line();

		println!("Last Name?");
		let last = read_line();

		println!("Email Address?");
		let address = read_line();

		let contact = Contact {
			email: address.to_string(),
			first_name: first.to_string(),
			last_name: last.to_string(),
		};

		contact
	}
}