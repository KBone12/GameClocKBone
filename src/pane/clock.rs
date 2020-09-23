use iced::{Column, Element};

pub struct Clock1PPane {}

impl Clock1PPane {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, _message: Clock1PMessage) {}

    pub fn view(&mut self) -> Element<Clock1PMessage> {
        Column::new().into()
    }
}

#[derive(Debug)]
pub enum Clock1PMessage {}
