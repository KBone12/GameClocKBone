use std::time::Duration;

#[derive(Clone, Debug)]
pub struct ClockSettings {
    pub time_limit: Duration,
}

impl Default for ClockSettings {
    fn default() -> Self {
        Self {
            time_limit: Duration::new(3 * 60, 0), // Default time limit is 3min.
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Settings {
    pub clock: ClockSettings,
}
