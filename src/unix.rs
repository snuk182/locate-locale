use std::env;

fn env(name: &str) -> String {
	if let Ok(loc) = env::var(name) {
		loc.split(".").next().unwrap().into()
	} else {
		String::new()
	}
}

pub fn system() -> String {
    env("LC_ALL")
}

pub fn user() -> String {
    env("LANG")
}    