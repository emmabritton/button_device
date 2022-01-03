mod config;

use std::borrow::BorrowMut;
use color_eyre::Result;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use clap::{App, Arg, crate_authors, crate_description, crate_name, crate_version};
use comm_lib::{get_best_match_device, LED_BLUE, LED_GREEN, LED_RED};
use comm_lib::manager::{DeviceManager, Update};
use crate::config::load_config;
use crate::config::rules::{NextExecution, Rules};

fn main() -> Result<()>{
    color_eyre::install()?;

    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(Arg::with_name("config")
            .default_value("config.json")
            .multiple(false)
            .help("Config file, see README.md for format info")
            .required(false)
            .takes_value(true)
            .value_name("FILE"))
        .get_matches();

    let config_file_path = matches.value_of("config").unwrap();
    let file = Path::new(config_file_path);
    if file.exists() {
        if let Ok(rules) = load_config(config_file_path) {
            run(rules)
        }
    } else {
        eprintln!("No file found at {}", config_file_path);
    }
    Ok(())
}

fn run(rules: Rules) {
    let board = get_best_match_device().expect("Unable to find device");
    let mut manager = DeviceManager::new(board);

    let mut next_execution = NextExecution::new();
    loop {
        if let Some(led) = &rules.led_red {
            if next_execution.is_red_led_ready() {
                match std::process::Command::new(&led.script)
                    .status() {
                    Ok(code) => manager.send(Update::LED(LED_RED, code.success())).unwrap(),
                    Err(err) => eprintln!("Error when executing Red LED script: {}", err)
                }
                next_execution.reset_red_led(led.freq.to_seconds());
            }
        }
        if let Some(led) = &rules.led_blue {
            if next_execution.is_blue_led_ready() {
                match std::process::Command::new(&led.script)
                    .status() {
                    Ok(code) => manager.send(Update::LED(LED_BLUE, code.success())).unwrap(),
                    Err(err) => eprintln!("Error when executing Blue LED script: {}", err)
                }
                next_execution.reset_blue_led(led.freq.to_seconds());
            }
        }
        if let Some(led) = &rules.led_green {
            if next_execution.is_green_led_ready() {
                match std::process::Command::new(&led.script)
                    .status() {
                    Ok(code) => manager.send(Update::LED(LED_GREEN, code.success())).unwrap(),
                    Err(err) => eprintln!("Error when executing Green LED script: {}", err)
                }
                next_execution.reset_green_led(led.freq.to_seconds());
            }
        }
        if let Some(display) = &rules.display {
            if next_execution.is_display_ready() {
                let output = std::process::Command::new(&display.script)
                    .output()
                    .unwrap()
                    .stdout;

                let output = String::from_utf8(output).unwrap();

                let mut text = String::new();

                for letter in output.chars().borrow_mut() {
                    if letter.is_ascii_graphic() || letter as u8 == 32 {
                       text.push(letter);
                    }
                }

                manager.send(Update::Text(text)).unwrap();

                next_execution.reset_display(display.freq.to_seconds());
            }
        }
        manager.recv().unwrap();
        let results = manager.get_button_state();
        if results[0] {
            if let Some(button) = &rules.button0 {
                std::process::Command::new(&button.script)
                    .spawn()
                    .unwrap();
            }
        }

        sleep(Duration::from_millis(400))
    }
}