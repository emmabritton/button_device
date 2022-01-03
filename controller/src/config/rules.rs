use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct NextExecution {
    led_blue: SystemTime,
    led_red: SystemTime,
    led_green: SystemTime,
    display: SystemTime,
}

impl NextExecution {
    pub fn new() -> Self {
        NextExecution { led_blue: UNIX_EPOCH, led_red: UNIX_EPOCH, led_green: UNIX_EPOCH, display: UNIX_EPOCH }
    }
}

impl NextExecution {
    pub fn is_red_led_ready(&self) -> bool {
        self.led_red < SystemTime::now()
    }

    pub fn reset_red_led(&mut self, seconds: u64) {
        self.led_red = SystemTime::now();
        self.led_red.checked_add(Duration::from_secs(seconds)).unwrap();
    }

    pub fn is_blue_led_ready(&self) -> bool {
        self.led_blue < SystemTime::now()
    }

    pub fn reset_blue_led(&mut self, seconds: u64) {
        self.led_blue = SystemTime::now();
        self.led_blue.checked_add(Duration::from_secs(seconds)).unwrap();
    }

    pub fn is_green_led_ready(&self) -> bool {
        self.led_green < SystemTime::now()
    }

    pub fn reset_green_led(&mut self, seconds: u64) {
        self.led_green = SystemTime::now();
        self.led_green.checked_add(Duration::from_secs(seconds)).unwrap();
    }

    pub fn is_display_ready(&self) -> bool {
        self.display < SystemTime::now()
    }

    pub fn reset_display(&mut self, seconds: u64) {
        self.display = SystemTime::now();
        self.display.checked_add(Duration::from_secs(seconds)).unwrap();
    }
}

pub struct Rules {
    pub led_green: Option<AutoScriptRules>,
    pub led_blue: Option<AutoScriptRules>,
    pub led_red: Option<AutoScriptRules>,
    pub display: Option<AutoScriptRules>,
    pub button0: Option<ExecuteScriptRules>,
    pub button1: Option<ExecuteScriptRules>,
    pub button2: Option<ExecuteScriptRules>,
    pub button3: Option<ExecuteScriptRules>,
}

impl Rules {
    pub fn new(led_green: Option<AutoScriptRules>, led_blue: Option<AutoScriptRules>, led_red: Option<AutoScriptRules>, display: Option<AutoScriptRules>, button0: Option<ExecuteScriptRules>, button1: Option<ExecuteScriptRules>, button2: Option<ExecuteScriptRules>, button3: Option<ExecuteScriptRules>) -> Self {
        Rules { led_green, led_blue, led_red, display, button0, button1, button2, button3 }
    }
}

pub struct ExecuteScriptRules {
    pub script: String,
    pub args: Vec<String>,
}

impl ExecuteScriptRules {
    pub fn new(script: String, args: Vec<String>) -> Self {
        ExecuteScriptRules { script, args }
    }
}

pub struct AutoScriptRules {
    pub script: String,
    pub args: Vec<String>,
    pub freq: Frequency
}

impl AutoScriptRules {
    pub fn new(script: String, args: Vec<String>, freq: Frequency) -> Self {
        AutoScriptRules { script, args, freq }
    }
}

pub struct Frequency {
    amount: usize,
    unit: Unit
}

impl Frequency {
    pub fn new(amount: usize, unit: Unit) -> Self {
        Frequency { amount, unit }
    }
}

impl Frequency {
    pub fn to_seconds(&self) -> u64 {
        (match self.unit {
            Unit::Seconds => self.amount,
            Unit::Minutes => self.amount * 60
        }) as u64
    }
}

pub enum Unit {
    Seconds,
    Minutes,
}