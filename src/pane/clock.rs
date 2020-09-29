use std::{
    any::TypeId,
    hash::{Hash, Hasher},
    time::{Duration, Instant},
};

use futures_timer::Delay;
use iced::{
    button, Align, Button, Color, Column, Container, Element, HorizontalAlignment, Length, Row,
    Subscription, Text, VerticalAlignment,
};
use iced_futures::{
    futures::{stream, StreamExt},
    subscription::Recipe,
    BoxStream,
};

pub struct ClockPane {
    panes: Vec<Clock1PPane>,
    toggle_button: button::State,
    pause_button: button::State,
    current_pane: usize,
    interval: Duration,
}

impl ClockPane {
    pub fn new<Ds: AsRef<[Duration]>>(time_limits: Ds) -> Self {
        let time_limits = time_limits.as_ref();
        let n = time_limits.len();
        assert!(n > 0 && n < 5);
        Self {
            panes: time_limits
                .iter()
                .copied()
                .map(|time_limit| Clock1PPane::new(time_limit))
                .collect(),
            toggle_button: button::State::new(),
            pause_button: button::State::new(),
            current_pane: n - 1,
            interval: Duration::from_millis(10),
        }
    }

    pub fn update(&mut self, message: ClockMessage) {
        self.panes[self.current_pane].update(message.clone());
        match message {
            ClockMessage::Pause => {
                self.panes[self.current_pane].running = false;
                if self.panes.len() != 1 {
                    // After resumed, the current clock will start.
                    self.current_pane =
                        (self.current_pane + self.panes.len() - 1) % self.panes.len();
                }
            }
            ClockMessage::Toggle => {
                if self.panes.len() == 1 {
                    let ref mut pane = self.panes[self.current_pane];
                    pane.running = !pane.running;
                    if pane.running {
                        pane.previous = Instant::now();
                    }
                } else {
                    self.panes[self.current_pane].running = false;
                    self.current_pane = (self.current_pane + 1) % self.panes.len();
                    self.panes[self.current_pane].running = true;
                    self.panes[self.current_pane].previous = Instant::now();
                }
            }
            _ => {}
        }
    }

    pub fn view(&mut self) -> Element<ClockMessage> {
        let to_text = |d: Duration| -> Text {
            let sec = d.as_secs();
            let hour = sec / (60 * 60);
            let sec = sec - hour * (60 * 60);
            let min = sec / 60;
            let sec = sec - min * 60;
            Text::new(format!("{:02}:{:02}:{:02}", hour, min, sec))
                .width(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center)
        };
        Container::new(
            Column::new()
                .push(
                    Button::new(
                        &mut self.toggle_button,
                        match self.panes.len() {
                            1 => {
                                let element: Element<_> =
                                    to_text(self.panes[self.current_pane].remaining).into();
                                element
                            }
                            2 => Row::with_children(
                                self.panes
                                    .iter()
                                    .map(|pane| to_text(pane.remaining).into())
                                    .collect(),
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_items(Align::Center)
                            .into(),
                            3 => Column::new()
                                .push(to_text(self.panes[0].remaining))
                                .push(
                                    Row::with_children(
                                        self.panes[1..]
                                            .iter()
                                            .map(|pane| to_text(pane.remaining).into())
                                            .collect(),
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .align_items(Align::Center),
                                )
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .align_items(Align::Center)
                                .into(),
                            4 => Column::new()
                                .push(
                                    Row::with_children(
                                        self.panes[0..2]
                                            .iter()
                                            .map(|pane| to_text(pane.remaining).into())
                                            .collect(),
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .align_items(Align::Center),
                                )
                                .push(
                                    Row::with_children(
                                        self.panes[2..]
                                            .iter()
                                            .map(|pane| to_text(pane.remaining).into())
                                            .collect(),
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .align_items(Align::Center),
                                )
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .align_items(Align::Center)
                                .into(),
                            _ => unimplemented!(),
                        },
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_press(ClockMessage::Toggle)
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
                    .on_press(ClockMessage::Pause),
                )
                .align_items(Align::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    pub fn subscription(&self) -> Subscription<ClockMessage> {
        Subscription::from_recipe(ClockRecipe(self.interval)).map(ClockMessage::Tick)
    }
}

struct Clock1PPane {
    remaining: Duration,
    previous: Instant,
    running: bool,
}

impl Clock1PPane {
    fn new(time_limit: Duration) -> Self {
        Self {
            remaining: time_limit,
            previous: Instant::now(),
            running: false,
        }
    }

    fn update(&mut self, message: ClockMessage) {
        match message {
            ClockMessage::Tick(now) => {
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
            _ => {}
        }
    }
}

#[derive(Clone, Debug)]
pub enum ClockMessage {
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
