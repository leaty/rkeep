use serde::Deserialize;
use shellexpand::full_with_context_no_errors;
use std::env;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub socket: PathBuf,
    pub session: Vec<Session>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Session {
    pub name: String,
    pub database: PathBuf,
    pub keyfile: Option<PathBuf>,
    pub alive: u32,
    pub clipboard: u32,
    pub command: Command,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Command {
    pub pass: Vec<String>,
    pub list: Vec<String>,
}

impl Config {
    #[inline(always)]
    pub fn process(mut self) -> Self {
        self.socket = expand_path(&self.socket);
        self.session.iter_mut().for_each(process_session);

        #[inline(always)]
        #[allow(deprecated)] // suppress `env::home_dir` warning (doesn't matter, since we target linux anyways)
        fn expand_path(path: &PathBuf) -> PathBuf {
            PathBuf::from(
                &(*full_with_context_no_errors(
                    path.to_str().expect("path contains invalid unicode"),
                    || {
                        env::home_dir().map(|path| {
                            path.to_str()
                                .expect("path contains invalid unicode")
                                .to_string()
                        })
                    },
                    |identifier| env::var(identifier).ok(),
                )),
            )
        }

        #[inline(always)]
        fn process_session(
            Session {
                name,
                ref mut database,
                ref mut keyfile,
                ref mut command,
                ..
            }: &mut Session,
        ) {
            *database = expand_path(&database);
            keyfile.as_mut().map(|keyfile| expand_path(keyfile));
            process_command(command, &name);
        }

        #[inline(always)]
        #[allow(deprecated)]
        fn process_command(cmd: &mut Command, session_name: &str) {
            cmd.pass
                .iter_mut()
                .chain(cmd.list.iter_mut())
                .for_each(|arg| {
                    *arg = String::from(full_with_context_no_errors(
                        &arg.replace("{session.name}", session_name),
                        || {
                            env::home_dir().map(|path| {
                                path.to_str()
                                    .expect("path contains invalid unicode")
                                    .to_string()
                            })
                        },
                        |identifier| env::var(identifier).ok(),
                    ))
                });
        }

        self
    }
}
