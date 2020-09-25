use std::time::Duration;

pub struct ClockSettings {
    time_limit: Duration,
}

impl ClockSettings {
    pub fn time_limit(&self) -> Duration {
        self.time_limit
    }
}

impl Default for ClockSettings {
    fn default() -> Self {
        Self {
            time_limit: Duration::new(3 * 60, 0), // Default time limit is 3min.
        }
    }
}

#[derive(Default)]
pub struct Settings {
    clock: ClockSettings,
}

impl Settings {
    pub fn clock(&self) -> &ClockSettings {
        &self.clock
    }
}
