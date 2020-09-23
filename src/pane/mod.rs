use std::collections::VecDeque;

use iced::{Column, Element, Sandbox};

pub mod clock;
use clock::{Clock1PMessage, Clock1PPane};

pub struct RootPane {
    children: VecDeque<Pane>,
}

impl Sandbox for RootPane {
    type Message = RootMessage;

    fn new() -> Self {
        let mut children = VecDeque::new();
        children.push_back(Pane::Clock1P(Clock1PPane::new()));
        Self { children }
    }

    fn title(&self) -> String {
        "GameClocKBone".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        if let Some(pane) = self.children.back_mut() {
            pane.update(message);
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        if let Some(pane) = self.children.back_mut() {
            pane.view()
        } else {
            Column::new().into()
        }
    }
}

#[derive(Debug)]
pub enum RootMessage {
    Clock1P(Clock1PMessage),
}

enum Pane {
    Clock1P(Clock1PPane),
}

impl Pane {
    fn update(&mut self, message: RootMessage) {
        match self {
            Pane::Clock1P(pane) => match message {
                RootMessage::Clock1P(message) => pane.update(message),
                _ => unimplemented!(),
            },
        }
    }

    fn view(&mut self) -> Element<RootMessage> {
        match self {
            Pane::Clock1P(pane) => pane.view().map(RootMessage::Clock1P),
        }
    }
}
