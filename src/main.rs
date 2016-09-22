extern crate hyper;

use hyper::*;
use std::io::Read;
use std::process::Command;

fn main() {
		let client = Client::new();

		let mut res = client.get("http://whatthecommit.com/index.txt").send().unwrap();
		match res.status {
			hyper::Ok => {
				let mut message = String::new();
				res.read_to_string(&mut message).unwrap();
				commit(&message);
			}
			_ => error()
		}
}

fn commit(message: &str) {
	let message = message.replace("\n", "");
	Command::new("git")
					.arg("commit")
					.arg("-m")
					.arg(&message)
					.spawn()
					.expect("Couldn't execute git commit.");
	println!("Successfully commited. Message: '{}'!", &message);
} 

fn error() {
	println!("Oh no! Something went wrong!");
	println!("Please ensure you have access to http://whatthecommit.com/index.txt");
} 