use std::{collections::HashMap, time::Duration};

use iced::{
    button, container, text_input, Align, Background, Button, Checkbox, Color, Column, Container,
    Element, HorizontalAlignment, Length, Radio, Row, Space, Subscription, Text, TextInput,
    VerticalAlignment,
};

use crate::settings::{ClockSettings, Settings};

pub struct SettingPane {
    red_button: button::State,
    blue_button: button::State,
    yellow_button: button::State,
    green_button: button::State,
    current_clock: Clock,
    clocks: HashMap<Clock, ClockSettings>,
    enabled: HashMap<Clock, bool>,
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
        let clocks: HashMap<_, _> = [Clock::Blue, Clock::Green, Clock::Red, Clock::Yellow]
            .iter()
            .map(|&clock| (clock, ClockSettings::default()))
            .collect();
        let mut enabled: HashMap<_, _> = clocks.keys().map(|&key| (key, false)).collect();
        *enabled.get_mut(&Clock::Blue).unwrap() = true;
        Self {
            red_button: button::State::new(),
            blue_button: button::State::new(),
            yellow_button: button::State::new(),
            green_button: button::State::new(),
            current_clock: Clock::Blue,
            clocks,
            enabled,
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
            SettingMessage::ClockChanged(clock) => {
                self.current_clock = clock;
                let sec = self.clocks[&self.current_clock].time_limit.as_secs();
                let hour = sec / (60 * 60);
                let sec = sec - hour * 60 * 60;
                let min = sec / 60;
                let sec = sec - min * 60;
                self.time_limit_hour_value = hour.to_string();
                self.time_limit_min_value = min.to_string();
                self.time_limit_sec_value = sec.to_string();
            }
            SettingMessage::ClockEnablingToggled(enabled) => {
                *self.enabled.get_mut(&self.current_clock).unwrap() = enabled;
            }
            SettingMessage::TimeLimitHourChanged(hour) => {
                if hour.is_empty() {
                    self.time_limit_hour_value = String::new();
                } else {
                    if hour.trim().parse::<u8>().is_ok() {
                        self.time_limit_hour_value = hour;
                        self.clocks.get_mut(&self.current_clock).unwrap().time_limit =
                            self.time_limit();
                    }
                }
            }
            SettingMessage::TimeLimitMinChanged(min) => {
                if min.is_empty() {
                    self.time_limit_min_value = String::new();
                } else {
                    if min.trim().parse::<u16>().is_ok() {
                        self.time_limit_min_value = min;
                        self.clocks.get_mut(&self.current_clock).unwrap().time_limit =
                            self.time_limit();
                    }
                }
            }
            SettingMessage::TimeLimitSecChanged(sec) => {
                if sec.is_empty() {
                    self.time_limit_sec_value = String::new();
                } else {
                    if sec.trim().parse::<u16>().is_ok() {
                        self.time_limit_sec_value = sec;
                        self.clocks.get_mut(&self.current_clock).unwrap().time_limit =
                            self.time_limit();
                    }
                }
            }
            _ => {}
        }
    }

    pub fn view(&mut self) -> Element<SettingMessage> {
        let done_button = Button::new(
            &mut self.done_button,
            Text::new("Done")
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center),
        );
        let done_button = if self.enabled.values().any(|b| *b) {
            done_button.on_press(SettingMessage::Done(Settings {
                clocks: self
                    .enabled
                    .iter()
                    .zip(self.clocks.values())
                    .filter_map(|((_, enabled), setting)| {
                        if *enabled {
                            Some(setting.clone())
                        } else {
                            None
                        }
                    })
                    .collect(),
                ..Settings::default()
            }))
        } else {
            done_button
        };
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
                                        Rule::AbsoluteTiming,
                                        "absolute timing",
                                        Some(Rule::AbsoluteTiming),
                                        SettingMessage::RuleSelected,
                                    ))
                                    .align_items(Align::Center),
                            )
                            .push(
                                Column::new()
                                    .push(
                                        Row::new()
                                            .push(
                                                Button::new(&mut self.blue_button, Text::new(""))
                                                    .on_press(SettingMessage::ClockChanged(
                                                        Clock::Blue,
                                                    ))
                                                    .width(Length::FillPortion(1))
                                                    .style(ColorButton(Clock::Blue.as_color())),
                                            )
                                            .push(
                                                Button::new(&mut self.green_button, Text::new(""))
                                                    .on_press(SettingMessage::ClockChanged(
                                                        Clock::Green,
                                                    ))
                                                    .width(Length::FillPortion(1))
                                                    .style(ColorButton(Clock::Green.as_color())),
                                            )
                                            .push(
                                                Button::new(&mut self.red_button, Text::new(""))
                                                    .on_press(SettingMessage::ClockChanged(
                                                        Clock::Red,
                                                    ))
                                                    .width(Length::FillPortion(1))
                                                    .style(ColorButton(Clock::Red.as_color())),
                                            )
                                            .push(
                                                Button::new(&mut self.yellow_button, Text::new(""))
                                                    .on_press(SettingMessage::ClockChanged(
                                                        Clock::Yellow,
                                                    ))
                                                    .width(Length::FillPortion(1))
                                                    .style(ColorButton(Clock::Yellow.as_color())),
                                            )
                                            .push(Space::with_width(Length::FillPortion(12)))
                                            .width(Length::Fill)
                                            .align_items(Align::Start),
                                    )
                                    .push(
                                        Container::new(
                                            Column::new()
                                                .push(Checkbox::new(
                                                    self.enabled[&self.current_clock],
                                                    "enabled",
                                                    SettingMessage::ClockEnablingToggled,
                                                ))
                                                .push(
                                                    Row::new()
                                                        .push(
                                                            Text::new("time limit: ")
                                                                .vertical_alignment(
                                                                    VerticalAlignment::Center,
                                                                ),
                                                        )
                                                        .push(TextInput::new(
                                                            &mut self.time_limit_hour_input,
                                                            "hour",
                                                            &self.time_limit_hour_value,
                                                            SettingMessage::TimeLimitHourChanged,
                                                        ))
                                                        .push(Text::new(":").vertical_alignment(
                                                            VerticalAlignment::Center,
                                                        ))
                                                        .push(TextInput::new(
                                                            &mut self.time_limit_min_input,
                                                            "minute",
                                                            &self.time_limit_min_value,
                                                            SettingMessage::TimeLimitMinChanged,
                                                        ))
                                                        .push(Text::new(":").vertical_alignment(
                                                            VerticalAlignment::Center,
                                                        ))
                                                        .push(TextInput::new(
                                                            &mut self.time_limit_sec_input,
                                                            "second",
                                                            &self.time_limit_sec_value,
                                                            SettingMessage::TimeLimitSecChanged,
                                                        )),
                                                ),
                                        )
                                        .align_x(Align::Center)
                                        .center_x()
                                        .center_y()
                                        .style(ColorContainer(self.current_clock.as_color())),
                                    ),
                            )
                            .align_items(Align::Center),
                    )
                    .center_x()
                    .center_y(),
                )
                .push(done_button)
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

    fn time_limit(&self) -> Duration {
        Duration::new(
            self.time_limit_hour_value
                .trim()
                .parse::<u64>()
                .unwrap_or(0)
                * 60
                * 60
                + self.time_limit_min_value.trim().parse::<u64>().unwrap_or(0) * 60
                + self.time_limit_sec_value.trim().parse::<u64>().unwrap_or(0),
            0,
        )
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Clock {
    Blue,
    Green,
    Red,
    Yellow,
}

impl Clock {
    pub fn as_color(&self) -> Color {
        match self {
            Clock::Blue => Color::from_rgb(0.0, 0.0, 1.0),
            Clock::Green => Color::from_rgb(0.0, 1.0, 0.0),
            Clock::Red => Color::from_rgb(1.0, 0.0, 0.0),
            Clock::Yellow => Color::from_rgb(1.0, 1.0, 0.0),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    AbsoluteTiming,
}

#[derive(Clone, Debug)]
pub enum SettingMessage {
    RuleSelected(Rule),
    ClockChanged(Clock),
    ClockEnablingToggled(bool),
    TimeLimitHourChanged(String),
    TimeLimitMinChanged(String),
    TimeLimitSecChanged(String),
    Done(Settings),
}

struct ColorButton(Color);

impl button::StyleSheet for ColorButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(self.0)),
            ..button::Style::default()
        }
    }
}

struct ColorContainer(Color);

impl container::StyleSheet for ColorContainer {
    fn style(&self) -> container::Style {
        container::Style {
            border_color: self.0,
            border_width: 1,
            ..container::Style::default()
        }
    }
}
