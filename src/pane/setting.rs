use iced::{
    button, text_input, Align, Button, Column, Container, Element, HorizontalAlignment, Length,
    Radio, Row, Subscription, Text, TextInput, VerticalAlignment,
};
use std::time::Duration;

use crate::settings::{ClockSettings, Settings};

pub struct SettingPane {
    done_button: button::State,
    time_limit_hour_input: text_input::State,
    time_limit_hour_value: String,
    time_limit_min_input: text_input::State,
    time_limit_min_value: String,
    time_limit_sec_input: text_input::State,
    time_limit_sec_value: String,
}

impl SettingPane {
    pub fn new() -> Self {
        Self {
            done_button: button::State::new(),
            time_limit_hour_input: text_input::State::new(),
            time_limit_hour_value: String::new(),
            time_limit_min_input: text_input::State::new(),
            time_limit_min_value: String::new(),
            time_limit_sec_input: text_input::State::new(),
            time_limit_sec_value: String::new(),
        }
    }

    pub fn update(&mut self, message: SettingMessage) {
        match message {
            SettingMessage::TimeLimitHourChanged(hour) => {
                if hour.is_empty() {
                    self.time_limit_hour_value = String::new();
                } else {
                    if hour.trim().parse::<u8>().is_ok() {
                        self.time_limit_hour_value = hour;
                    }
                }
            }
            SettingMessage::TimeLimitMinChanged(min) => {
                if min.is_empty() {
                    self.time_limit_min_value = String::new();
                } else {
                    if min.trim().parse::<u16>().is_ok() {
                        self.time_limit_min_value = min;
                    }
                }
            }
            SettingMessage::TimeLimitSecChanged(sec) => {
                if sec.is_empty() {
                    self.time_limit_sec_value = String::new();
                } else {
                    if sec.trim().parse::<u16>().is_ok() {
                        self.time_limit_sec_value = sec;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn view(&mut self) -> Element<SettingMessage> {
        Container::new(
            Column::new()
                .push(
                    Text::new("GameClocKBone")
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
                )
                .push(
                    Container::new(
                        Column::new()
                            .push(
                                Row::new()
                                    .push(Radio::new(
                                        Players::One,
                                        "1",
                                        Some(Players::One),
                                        SettingMessage::PlayersSelected,
                                    ))
                                    .align_items(Align::Center),
                            )
                            .push(
                                Row::new()
                                    .push(Radio::new(
                                        Rule::AbsoluteTiming,
                                        "absolute timing",
                                        Some(Rule::AbsoluteTiming),
                                        SettingMessage::RuleSelected,
                                    ))
                                    .align_items(Align::Center),
                            )
                            .push(
                                Row::new()
                                    .push(
                                        Text::new("time limit: ")
                                            .vertical_alignment(VerticalAlignment::Center),
                                    )
                                    .push(TextInput::new(
                                        &mut self.time_limit_hour_input,
                                        "hour",
                                        &self.time_limit_hour_value,
                                        SettingMessage::TimeLimitHourChanged,
                                    ))
                                    .push(
                                        Text::new(":")
                                            .vertical_alignment(VerticalAlignment::Center),
                                    )
                                    .push(TextInput::new(
                                        &mut self.time_limit_min_input,
                                        "minute",
                                        &self.time_limit_min_value,
                                        SettingMessage::TimeLimitMinChanged,
                                    ))
                                    .push(
                                        Text::new(":")
                                            .vertical_alignment(VerticalAlignment::Center),
                                    )
                                    .push(TextInput::new(
                                        &mut self.time_limit_sec_input,
                                        "second",
                                        &self.time_limit_sec_value,
                                        SettingMessage::TimeLimitSecChanged,
                                    )),
                            )
                            .align_items(Align::Center),
                    )
                    .center_x()
                    .center_y(),
                )
                .push(
                    Button::new(
                        &mut self.done_button,
                        Text::new("Done")
                            .horizontal_alignment(HorizontalAlignment::Center)
                            .vertical_alignment(VerticalAlignment::Center),
                    )
                    .on_press(SettingMessage::Done(Settings {
                        clock: ClockSettings {
                            time_limit: Duration::new(
                                self.time_limit_hour_value
                                    .trim()
                                    .parse::<u64>()
                                    .unwrap_or(0)
                                    * (60 * 60)
                                    + self.time_limit_min_value.trim().parse::<u64>().unwrap_or(0)
                                        * 60
                                    + self.time_limit_sec_value.trim().parse::<u64>().unwrap_or(0),
                                0,
                            ),
                        },
                        ..Settings::default()
                    })),
                )
                .align_items(Align::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    pub fn subscription(&self) -> Subscription<SettingMessage> {
        Subscription::none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Players {
    One,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    AbsoluteTiming,
}

#[derive(Clone, Debug)]
pub enum SettingMessage {
    PlayersSelected(Players),
    RuleSelected(Rule),
    TimeLimitHourChanged(String),
    TimeLimitMinChanged(String),
    TimeLimitSecChanged(String),
    Done(Settings),
}
