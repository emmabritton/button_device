use std::path::Path;
use is_executable::is_executable;
use serde::Deserialize;
use crate::config::rules::{AutoScriptRules, ExecuteScriptRules, Frequency, Rules, Unit};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default = "Output::default")]
    pub output: Output,
    pub leds: Option<Leds>,
    pub display: Option<Script>,
    pub buttons: Option<Vec<Button>>,
}

impl Config {
    pub fn build_rules(&self) -> Result<Rules, Vec<String>> {
        let mut errors = vec![];

        if let Some(leds) = &self.leds {
            validate_led("Green", &leds.green, &mut errors);
            validate_led("Blue", &leds.blue, &mut errors);
            validate_led("Red", &leds.red, &mut errors);
        }

        if let Some(buttons) = &self.buttons {
            if buttons.len() > 4 {
                errors.push("Too many button scripts, max of 4 is supported".to_owned());
            }

            validate_button("0", &buttons.get(0), &mut errors);
            validate_button("1", &buttons.get(1), &mut errors);
            validate_button("2", &buttons.get(2), &mut errors);
            validate_button("3", &buttons.get(3), &mut errors);
        }

        validate_display(&self.display, &mut errors);

        if errors.is_empty() {
            Ok(Rules::new(
                make_script_rules(&self.leds.as_ref().and_then(|leds| leds.green.as_ref())),
                make_script_rules(&self.leds.as_ref().and_then(|leds| leds.blue.as_ref())),
                make_script_rules(&self.leds.as_ref().and_then(|leds| leds.red.as_ref())),
                make_script_rules(&self.display.as_ref()),
                make_button_rules(&self.buttons.as_ref().and_then(|buttons| buttons.get(0))),
                make_button_rules(&self.buttons.as_ref().and_then(|buttons| buttons.get(1))),
                make_button_rules(&self.buttons.as_ref().and_then(|buttons| buttons.get(2))),
                make_button_rules(&self.buttons.as_ref().and_then(|buttons| buttons.get(3))),
            ))
        } else {
            Err(errors)
        }
    }
}

fn make_script_rules(script: &Option<&Script>) -> Option<AutoScriptRules> {
    script.map(|script| {
        AutoScriptRules::new(
            script.script.clone(),
            script.args.clone().unwrap_or_default(),
            Frequency::new(script.freq_amount, script.freq_unit.into())
        )
    })
}

fn make_button_rules(button: &Option<&Button>) -> Option<ExecuteScriptRules> {
    button.map(|button| {
        ExecuteScriptRules::new(
            button.script.clone(),
            button.args.clone().unwrap_or_default()
        )
    })
}

fn validate_display(display: &Option<Script>, errors: &mut Vec<String>) {
    if let Some(display) = display {
        if display.freq_amount == 0 {
            errors.push("Display freq_amount is 0, min is 1".to_owned());
        }
        if display.freq_amount > 59 {
            errors.push(format!("Display freq_amount is {}, max is 59", display.freq_amount));
        }
        validate_script("Display", &display.script, errors);
    }
}

fn validate_led(name: &str, led: &Option<Script>, errors: &mut Vec<String>) {
    if let Some(led) = led {
        if led.freq_amount == 0 {
            errors.push(format!("{} LED freq_amount is 0, min is 1", name));
        }
        if led.freq_amount > 59 {
            errors.push(format!("{} LED freq_amount is {}, max is 59", name, led.freq_amount));
        }
        validate_script(&format!("{} LED", name), &led.script, errors);
    }
}

fn validate_button(name: &str, button: &Option<&Button>, errors: &mut Vec<String>) {
    if let Some(button) = button {
        validate_script(&format!("Button {}", name), &button.script, errors);
    }
}

fn validate_script(name: &str, path: &str, errors: &mut Vec<String>) {
    if !Path::new(path).exists() {
        errors.push(format!("Script for {} does not exist", name));
    } else if !Path::new(path).is_file() {
        errors.push(format!("Script for {} is not a file", name));
    } else if !is_executable(path) {
        errors.push(format!("Script for {} is not executable", name));
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Output {
    All,
    Debug,
    None,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Button {
    pub script: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Leds {
    pub green: Option<Script>,
    pub blue: Option<Script>,
    pub red: Option<Script>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Script {
    pub script: String,
    pub args: Option<Vec<String>>,
    #[serde(default = "one")]
    pub freq_amount: usize,
    pub freq_unit: FreqUnit,
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FreqUnit {
    Seconds,
    Minutes,
}

impl Default for FreqUnit {
    fn default() -> Self {
        FreqUnit::Minutes
    }
}

impl Default for Output {
    fn default() -> Self {
        Output::All
    }
}

fn one() -> usize {
    1
}

impl Into<Unit> for FreqUnit {
    fn into(self) -> Unit {
        match self {
            FreqUnit::Seconds => Unit::Seconds,
            FreqUnit::Minutes => Unit::Minutes
        }
    }
}