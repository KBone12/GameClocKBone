use std::{
    any::TypeId,
    hash::{Hash, Hasher},
    time::{Duration, Instant},
};

use futures_timer::Delay;
use iced::{
    button, Align, Button, Color, Column, Container, Element, HorizontalAlignment, Length,
    Subscription, Text, VerticalAlignment,
};
use iced_futures::{
    futures::{stream, StreamExt},
    subscription::Recipe,
    BoxStream,
};

pub struct Clock1PPane {
    remaining: Duration,
    previous: Instant,
    toggle_button: button::State,
    pause_button: button::State,
    running: bool,
}

impl Clock1PPane {
    pub fn new() -> Self {
        Self {
            remaining: Duration::new(5, 0),
            previous: Instant::now(),
            toggle_button: button::State::new(),
            pause_button: button::State::new(),
            running: false,
        }
    }

    pub fn update(&mut self, message: Clock1PMessage) {
        match message {
            Clock1PMessage::Pause => {
                self.running = false;
            }
            Clock1PMessage::Tick(now) => {
                if self.running {
                    if now < self.previous {
                        self.previous = now;
                    } else {
                        let dt = now - self.previous;
                        self.previous = now;
                        if self.remaining > dt {
                            self.remaining -= dt;
                        } else {
                            self.remaining = Duration::new(0, 0);
                        }
                    }
                }
            }
            Clock1PMessage::Toggle => {
                self.running = !self.running;
                if self.running {
                    self.previous = Instant::now();
                }
            }
        }
    }

    pub fn view(&mut self) -> Element<Clock1PMessage> {
        let sec = self.remaining.as_secs();
        let hour = sec / (60 * 60);
        let sec = sec - hour * (60 * 60);
        let min = sec / 60;
        let sec = sec - min * 60;
        Container::new(
            Column::new()
                .push(
                    Button::new(
                        &mut self.toggle_button,
                        Text::new(format!("{:02}:{:02}:{:02}", hour, min, sec))
                            .width(Length::Fill)
                            .horizontal_alignment(HorizontalAlignment::Center)
                            .vertical_alignment(VerticalAlignment::Center),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_press(Clock1PMessage::Toggle)
                    .style(ClockStyle),
                )
                .push(
                    Button::new(
                        &mut self.pause_button,
                        Text::new("pause")
                            .width(Length::Fill)
                            .horizontal_alignment(HorizontalAlignment::Center),
                    )
                    .width(Length::Fill)
                    .on_press(Clock1PMessage::Pause),
                )
                .align_items(Align::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    pub fn subscription(&self) -> Subscription<Clock1PMessage> {
        Subscription::from_recipe(ClockRecipe(Duration::from_millis(10))).map(Clock1PMessage::Tick)
    }
}

#[derive(Clone, Debug)]
pub enum Clock1PMessage {
    Pause,
    Tick(Instant),
    Toggle,
}

pub struct ClockRecipe(Duration);

impl<H: Hasher, E> Recipe<H, E> for ClockRecipe {
    type Output = Instant;

    fn hash(&self, state: &mut H) {
        TypeId::of::<Self>().hash(state);
        self.0.hash(state);
    }

    fn stream(self: Box<Self>, _input: BoxStream<E>) -> BoxStream<Self::Output> {
        stream::unfold(self.0, |d| async move {
            Delay::new(d).await;
            let now = Instant::now();
            Some((now, d))
        })
        .boxed()
    }
}

pub struct ClockStyle;

impl button::StyleSheet for ClockStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: None,
            text_color: Color::BLACK,
            ..button::Style::default()
        }
    }
}
