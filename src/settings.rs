use std::time::Duration;

#[derive(Clone, Debug)]
pub struct ClockSettings {
    pub time_limit: Duration,
}

impl Default for ClockSettings {
    fn default() -> Self {
        Self {
            time_limit: Duration::new(0, 0), // Default time limit is 3min.
        }
    }
}

#[derive(Clone, Debug)]
pub struct Settings {
    pub clocks: Vec<ClockSettings>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            clocks: vec![ClockSettings::default()],
        }
    }
}
