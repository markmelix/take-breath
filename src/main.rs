mod lib;

use std::fs::{self, File};
use std::io::Write;

use lib::config::*;
use lib::counter::*;

const CONFIG_DIR: &str = "take-breath";
const CONFIG_FILE: &str = "config.toml";
const CONFIG_COMMENT_STRING: &str = r#"
# Configuration for Take Breath
# See https://github.com/markmelix/take-breath#customization"#;

fn main() {
    let mut config = Config::default();

    match dirs::config_dir() {
	Some(mut path) => {
	    path.push(CONFIG_DIR);

	    if let false = path.as_path().exists() {
		if let Err(e) = fs::create_dir_all(path.clone()) {
		    eprintln!("Failed to create config directory: {}", e);
		};
	    }

	    path.push(CONFIG_FILE);

	    match path.as_path().exists() {
		true => config = Config::apply_from_file(path),
		false => {
		    let mut config_file = match File::create(path) {
			Ok(file) => file,
			Err(e) => {
			    eprintln!("Failed to create {} file: {}", CONFIG_FILE, e);
			    return;
			}
		    };
		    let config_toml = match toml::to_string_pretty(&Config::default()) {
			Ok(string) => string,
			Err(e) => {
			    eprintln!("Failed to format config into the toml format:  {}", e);
			    return;
			}
		    };
		    write!(&mut config_file, "{}\n\n{}", CONFIG_COMMENT_STRING, config_toml.as_bytes());
		    // if let Err(e) = config_file.write_all(config_toml.as_bytes()) {
		    //	eprintln!("Failed to write config data to the file: {}", e);
		    // }
		}
	    }
	}
	None => eprintln!("Error: config directory not found"),
    }

    config.apply_env();

    let work = Work::new(
	config.work_time.duration,
	config.work_time.idle_to_pause,
    );

    let rest = Rest::new(
	config.rest_time.duration,
    );

    loop {
	work.count();
	rest.count();
    }
}
