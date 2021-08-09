mod lib;

#[cfg(not(feature = "config"))]
use lib::*;
use lib::counter::*;

#[cfg(feature = "config")]
const CONFIG_DIR: &str = "take-breath";
#[cfg(feature = "config")]
const CONFIG_FILE: &str = "config.toml";
#[cfg(feature = "config")]
const CONFIG_COMMENT_STRING: &str = r#"# Configuration for Take Breath
# See https://github.com/markmelix/take-breath#customization"#;

fn main() {
    #[cfg(feature = "config")]
    let config = get_config();

    #[cfg(feature = "config")]
    let work = Work::new(
	config.work_time.duration,
	config.work_time.idle_to_pause,
    );

    #[cfg(feature = "config")]
    let rest = Rest::new(
	config.rest_time.duration,
    );

    #[cfg(debug_assertions)]
    #[cfg(feature = "config")]
    println!("{:#?}", config);

    #[cfg(not(feature = "config"))]
    let work = Work::new(
	DEFAULT_WORK_DUR,
	DEFAULT_WORK_IDLE_TO_PAUSE,
    );

    #[cfg(not(feature = "config"))]
    let rest = Rest::new(
	DEFAULT_REST_DUR,
    );

    loop {
	work.count();
	rest.count();
    }
}


#[cfg(feature = "config")]
fn get_config() -> lib::config::Config {
    use std::fs::{self, File};
    use std::io::Write;
    use lib::config::*;

    let mut config = Config::default();

    for _ in 0..1 {
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
				break;
			    }
			};
			let config_toml = match toml::to_string_pretty(&Config::default()) {
			    Ok(string) => string,
			    Err(e) => {
				eprintln!("Failed to format config into the toml format:  {}", e);
				break;
			    }
			};
			if let Err(e) = write!(&mut config_file, "{}\n\n{}", CONFIG_COMMENT_STRING, config_toml) {
			    eprintln!("Failed to write config data to the file: {}", e);
			    break;
			}
		    }
		}
	    }
	    None => eprintln!("Error: config directory not found"),
	}
    }

    config
}
