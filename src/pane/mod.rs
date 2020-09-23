use std::collections::VecDeque;

use iced::{executor, Application, Column, Command, Element, Subscription};
use log::debug;

pub mod clock;
use clock::{Clock1PMessage, Clock1PPane};

pub struct RootPane {
    children: VecDeque<Pane>,
}

impl Application for RootPane {
    type Executor = executor::Default;
    type Message = RootMessage;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut children = VecDeque::new();
        children.push_back(Pane::Clock1P(Clock1PPane::new()));
        (Self { children }, Command::none())
    }

    fn title(&self) -> String {
        "GameClocKBone".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        debug!("Update RootPane (message: {:?})", message);

        if let Some(pane) = self.children.back_mut() {
            pane.update(message);
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        if let Some(pane) = self.children.back_mut() {
            pane.view()
        } else {
            Column::new().into()
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        if let Some(pane) = self.children.back() {
            pane.subscription()
        } else {
            Subscription::none()
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

    fn subscription(&self) -> Subscription<RootMessage> {
        match self {
            Pane::Clock1P(pane) => pane.subscription().map(RootMessage::Clock1P),
        }
    }
}
