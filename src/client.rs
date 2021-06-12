use clap::clap_app;
use clap::crate_authors;
use rkeep::Config;
use std::fs;
use std::io::Write;
use std::os::unix::net::UnixStream;

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
	let args = clap_app!(rkeep =>
		(version: VERSION)
		(author: crate_authors!())
		(about: "Simple client for rkeepd")
		(@arg c: -c --config <FILE> +takes_value default_value(default) "Configuration file")
		(@arg s: -s --session <NAME> +takes_value "Session name")
	)
	.get_matches();

	// Load config
	let config_file = args.value_of("c").unwrap_or(&default);
	let config_str = fs::read_to_string(&config_file)?;
	let config: Config = toml::from_str(&config_str).unwrap();

	let session = args.value_of("s").unwrap();

	let mut stream = UnixStream::connect(config.socket)?;
	stream.write_all(format!("{}|exec", session).as_bytes())?;

	Ok(())
}
