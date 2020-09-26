use std::collections::VecDeque;

use iced::{executor, Application, Column, Command, Element, Subscription};
use log::debug;

use crate::settings::Settings;

mod clock;
mod pause;
mod setting;

use clock::{Clock1PMessage, Clock1PPane};
use pause::{PauseMessage, PausePane};
use setting::{SettingMessage, SettingPane};

pub struct RootPane {
    children: VecDeque<Pane>,
    settings: Settings,
}

impl Application for RootPane {
    type Executor = executor::Default;
    type Message = RootMessage;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut children = VecDeque::new();
        let settings = Settings::default();
        children.push_back(Pane::Setting(SettingPane::new()));
        (Self { children, settings }, Command::none())
    }

    fn title(&self) -> String {
        "GameClocKBone".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        debug!("Update RootPane (message: {:?})", message);

        if let Some(pane) = self.children.back_mut() {
            pane.update(message.clone());
        }

        match message {
            Self::Message::Clock1P(message) => match message {
                Clock1PMessage::Pause => {
                    self.children.push_back(Pane::Pause(PausePane::new()));
                }
                _ => {}
            },
            Self::Message::Pause(message) => match message {
                PauseMessage::Settings => {
                    self.children.clear();
                    self.children.push_back(Pane::Setting(SettingPane::new()));
                }
                PauseMessage::Back => {
                    self.children.pop_back();
                }
                PauseMessage::Reset => {
                    self.children.clear();
                    self.children.push_back(Pane::Clock1P(Clock1PPane::new(
                        self.settings.clock.time_limit,
                    )));
                }
            },
            Self::Message::Setting(message) => match message {
                SettingMessage::Done(settings) => {
                    self.settings = settings;
                    self.children.clear();
                    self.children.push_back(Pane::Clock1P(Clock1PPane::new(
                        self.settings.clock.time_limit,
                    )));
                }
                _ => {}
            },
        };

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

#[derive(Clone, Debug)]
pub enum RootMessage {
    Clock1P(Clock1PMessage),
    Pause(PauseMessage),
    Setting(SettingMessage),
}

enum Pane {
    Clock1P(Clock1PPane),
    Pause(PausePane),
    Setting(SettingPane),
}

impl Pane {
    fn update(&mut self, message: RootMessage) {
        match self {
            Pane::Clock1P(pane) => {
                if let RootMessage::Clock1P(message) = message {
                    pane.update(message);
                }
            }
            Pane::Pause(pane) => {
                if let RootMessage::Pause(message) = message {
                    pane.update(message);
                }
            }
            Pane::Setting(pane) => {
                if let RootMessage::Setting(message) = message {
                    pane.update(message);
                }
            }
        }
    }

    fn view(&mut self) -> Element<RootMessage> {
        match self {
            Pane::Clock1P(pane) => pane.view().map(RootMessage::Clock1P),
            Pane::Pause(pane) => pane.view().map(RootMessage::Pause),
            Pane::Setting(pane) => pane.view().map(RootMessage::Setting),
        }
    }

    fn subscription(&self) -> Subscription<RootMessage> {
        match self {
            Pane::Clock1P(pane) => pane.subscription().map(RootMessage::Clock1P),
            Pane::Pause(pane) => pane.subscription().map(RootMessage::Pause),
            Pane::Setting(pane) => pane.subscription().map(RootMessage::Setting),
        }
    }
}
