pub mod sender {
use crate::Draft;
use crate::read_line;
use std::fmt::Display;
use std::fs;
use std::process::Command;

pub trait Deliver {
	fn full_send(&self);
}

impl <T: Display + PartialEq>Deliver for Draft<T>{
	fn full_send(&self) {
		println!("what is your gmail password?");
		let password = read_line();

		let node_file = format!(
		"var nodemailer = require('nodemailer');

		var transporter = nodemailer.createTransport({{
		  service: 'gmail',
		  auth: {{
		    user: '{}',
		    pass: '{}'
		  }}
		}});

		var mailOptions = {{
		  from: '{}',
		  to: '{}',
		  subject: '{}',
		  text: '{}'
		}};

		transporter.sendMail(mailOptions, function(error, info){{
		  if (error) {{
		    console.log(error);
		  }} else {{
		    console.log('Email sent: ' + info.response);
		  }}
		}});",self.sender, password.trim(), self.sender, self.reciever, self.subject, self.message);

		fs::write("gmail.js", node_file).unwrap();

		start_node();

		println!("email sent!!");
	}
}

fn start_node(){
Command::new("cmd")
        .args(["/C" , "node gmail.js"])
        .spawn()
        .expect("ls command failed to start");
}
}