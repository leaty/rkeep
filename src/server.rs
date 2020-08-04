mod interface;
mod config;
use interface::keepass;
use interface::rofi;
use clap::clap_app;
use git_version::git_version;
use std::collections::HashMap;
use std::os::unix::net::{UnixStream, UnixListener};
use std::io::{BufRead, BufReader};
use config::Config;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Default config path
	let xdgd = xdg::BaseDirectories::with_prefix("rkeep").unwrap();
	let xdgc = xdgd.place_config_file("config.toml").unwrap();
	let default = xdgc.to_str().unwrap();
	
	// Shell args
	let version = git_version!();
	let args = clap_app!(rkeep_server => 
		(version: version)
		(author: "leaty <dev@leaty.net>")
		(about: "Persistent Rofi backend for KeePassXC using Rust")
		(@arg c: -c --config <FILE> +takes_value default_value(default) "Configuration file")
	).get_matches();
	
	// Load config
	let config_file = args.value_of("c").unwrap_or(&default);
	let config_str = fs::read_to_string(&config_file)?;
	let config: Config = toml::from_str(&config_str).unwrap();

	// Set up sessions
	let mut sessions = HashMap::<String, keepass::Session>::new();
	for session in &config.session {
		sessions.insert(session.name.clone(), keepass::Session::new(&session));
	}

	// Start listening for clients
	let _ = fs::remove_file(&config.socket);
	let listener = UnixListener::bind(config.socket)?;
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				client(&mut sessions, stream)?;
			},
			Err(_) => {
				continue;
			}
		}
	}
	
	Ok(())
}

fn client(sessions: &mut HashMap<String, keepass::Session>, stream: UnixStream) -> Result<(), Box<dyn std::error::Error>> {
	let stream = BufReader::new(stream);
	for line in stream.lines() {
		let command = line.unwrap();
		let v: Vec<&str> = command.split("|").collect();
		let (s, c) = (v[0], v[1]);

		// Check if session exists by name
		if sessions.contains_key(s) {
			let session = &mut sessions.get_mut(s).unwrap();

			// Connect if not yet open
			if !session.connection.is_some() {
				if let Ok(password) = rofi::password() {
					if let Err(_) = session.open(&password) {
						break;
					}
				}
				else {
					break;
				}
			}
				
			match c {
				// Execute keepass backend and rofi frontend
				"exec" => {
					let entries = session.list()?;
					if let Ok(entry) = rofi::list(&session.name, &entries) {
						session.clip(&entry)?;
					}
					break;
				},
				_ => break,
			}
		}
	}

	Ok(())
}
