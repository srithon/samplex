use iced::{
    slider, button, Button, Align, Color, Column, Element, HorizontalAlignment, Length,
    Point, Row, Sandbox, Settings, Size, Slider, Text, Vector,
    VerticalAlignment,
};

#[derive(Default)]
pub struct SamplexApp {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for SamplexApp {
    type Message = Message;

    fn new() -> SamplexApp {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Samplex")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Self::Message::IncrementPressed => {
                self.value += 1;
            },
            Self::Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Text::new("Hello, world!")
            )
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}
