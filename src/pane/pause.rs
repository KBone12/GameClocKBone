use iced::{button, Button, Container, Element, Length, Row, Subscription, Text};

pub struct PausePane {
    settings_button: button::State,
    back_button: button::State,
    reset_button: button::State,
}

impl PausePane {
    pub fn new() -> Self {
        Self {
            settings_button: button::State::new(),
            back_button: button::State::new(),
            reset_button: button::State::new(),
        }
    }

    pub fn update(&mut self, _message: PauseMessage) {}

    pub fn view(&mut self) -> Element<PauseMessage> {
        Container::new(
            Row::new()
                .push(
                    Button::new(&mut self.settings_button, Text::new("Settings"))
                        .on_press(PauseMessage::Settings),
                )
                .push(
                    Button::new(&mut self.back_button, Text::new("Back"))
                        .on_press(PauseMessage::Back),
                )
                .push(
                    Button::new(&mut self.reset_button, Text::new("Reset"))
                        .on_press(PauseMessage::Reset),
                ),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    pub fn subscription(&self) -> Subscription<PauseMessage> {
        Subscription::none()
    }
}

#[derive(Clone, Debug)]
pub enum PauseMessage {
    Settings,
    Back,
    Reset,
}
