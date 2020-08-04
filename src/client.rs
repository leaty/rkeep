mod config;
use clap::clap_app;
use git_version::git_version;
use std::os::unix::net::UnixStream;
use config::Config;
use std::fs;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Default config path
	let xdgd = xdg::BaseDirectories::with_prefix("rkeep").unwrap();
	let xdgc = xdgd.place_config_file("config.toml").unwrap();
	let default = xdgc.to_str().unwrap();
	
	// Shell args
	let version = git_version!();
	let args = clap_app!(rkeep_client => 
		(version: version)
		(author: "leaty <dev@leaty.net>")
		(about: "Persistent Rofi backend for KeePassXC using Rust")
		(@arg c: -c --config <FILE> +takes_value default_value(default) "Configuration file")
		(@arg s: -s --session <NAME> +takes_value "Session name")
	).get_matches();
	
	// Load config
	let config_file = args.value_of("c").unwrap_or(&default);
	let config_str = fs::read_to_string(&config_file)?;
	let config: Config = toml::from_str(&config_str).unwrap();
	
	let session = args.value_of("s").unwrap();

	let mut stream = UnixStream::connect(config.socket)?;
	stream.write_all(format!("{}|exec", session).as_bytes())?;

	Ok(())
}

