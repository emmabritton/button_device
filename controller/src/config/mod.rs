mod config;
pub mod rules;

use std::fs;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use crate::config::config::Config;
use crate::config::rules::Rules;

pub fn load_config(path: &str) -> Result<Rules> {
    match fs::read_to_string(path) {
        Ok(contents) => match serde_json::from_str::<Config>(&contents) {
            Ok(config) => {
                return match config.build_rules() {
                    Ok(rules) => Ok(rules),
                    Err(e) => {
                        eprintln!("Config errors:");
                        for line in e {
                            eprintln!("{}", line);
                        }
                        Err(eyre!(""))
                    }
                }
            },
            Err(err) => eprintln!("Unable to parse config file: {}", err)
        },
        Err(err) => eprintln!("Unable to read config file: {}", err)
    }
    Err(eyre!(""))
}