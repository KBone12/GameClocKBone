use iced::{Column, Element, Text};
use std::time::Duration;

pub struct Clock1PPane {
    remaining: Duration,
}

impl Clock1PPane {
    pub fn new() -> Self {
        Self {
            remaining: Duration::new(0, 0),
        }
    }

    pub fn update(&mut self, _message: Clock1PMessage) {}

    pub fn view(&mut self) -> Element<Clock1PMessage> {
        let sec = self.remaining.as_secs();
        let hour = sec / (60 * 60);
        let sec = sec - hour * (60 * 60);
        let min = sec / 60;
        let sec = sec - min * 60;
        Column::new()
            .push(Text::new(format!("{:02}:{:02}:{:02}", hour, min, sec)))
            .into()
    }
}

#[derive(Debug)]
pub enum Clock1PMessage {}
