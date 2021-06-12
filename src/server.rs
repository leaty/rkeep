use clap::clap_app;
use clap::crate_authors;
use rkeep::interface::{display, keepass};
use rkeep::Config;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(debug_assertions)]
use git_version::git_version;
#[cfg(debug_assertions)]
pub const VERSION: &'static str = git_version!();

#[cfg(not(debug_assertions))]
use clap::crate_version;
#[cfg(not(debug_assertions))]
pub const VERSION: &'static str = crate_version!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Default config path
	let xdgd = xdg::BaseDirectories::with_prefix("rkeep").unwrap();
	let xdgc = xdgd.place_config_file("config.toml").unwrap();
	let default = xdgc.to_str().unwrap();

	// Shell args
	let args = clap_app!(rkeepd =>
		(version: VERSION)
		(author: crate_authors!())
		(about: "Persistent KeePass backend with display hooks")
		(@arg c: -c --config <FILE> +takes_value default_value(default) "Configuration file")
	)
	.get_matches();

	// Load config
	let config_file = args.value_of("c").unwrap_or(&default);
	let config_str = fs::read_to_string(&config_file)?;
	let mut config: Config = toml::from_str(&config_str).unwrap();

	// Set up sessions
	let sessions = Arc::new(Mutex::new(HashMap::<String, keepass::Session>::new()));
	for session in &mut config.session {
		session.parse();
		sessions
			.lock()
			.unwrap()
			.insert(session.name.clone(), keepass::Session::new(&session));
	}

	// Set up cleaning thread
	cleaning(Arc::clone(&sessions));

	// Start listening for clients
	let _ = fs::remove_file(&config.socket);
	let listener = UnixListener::bind(config.socket)?;
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				if let Err(e) = client(&mut sessions.lock().unwrap(), stream) {
					println!("{}", e.to_string());
				}
			}
			Err(_) => {
				continue;
			}
		}
	}

	Ok(())
}

fn cleaning(sessions: Arc<Mutex<HashMap<String, keepass::Session>>>) {
	thread::spawn(move || loop {
		for (_, session) in sessions.lock().unwrap().iter_mut() {
			if let Err(e) = session.clean() {
				println!("{}", e.to_string());
			}
		}

		thread::sleep(Duration::new(1, 0));
	});
}

fn client(
	sessions: &mut HashMap<String, keepass::Session>,
	stream: UnixStream,
) -> Result<(), Box<dyn std::error::Error>> {
	let stream = BufReader::new(stream);
	for line in stream.lines() {
		let command = line.unwrap();
		let v: Vec<&str> = command.split("|").collect();
		let (s, c) = (v[0], v[1]);

		// Check if session exists by name
		if sessions.contains_key(s) {
			let session = &mut sessions.get_mut(s).unwrap();

			// Open if not yet open
			if !session.database.is_some() {
				session.open(&display::password(&session.command.pass)?)?;
			}

			match c {
				// Execute keepass backend and display frontend
				"exec" => {
					let list = session.list()?;
					let entry = display::list(&session.command.list, &list)?;
					session.clip(&entry)?;

					// Since the user just clipped a new password
					// reset clipboard timeouts on other sessions
					for (key, other) in sessions {
						if key != s {
							other.clip_reset();
						}
					}

					break;
				}
				_ => break,
			}
		}
	}

	Ok(())
}
