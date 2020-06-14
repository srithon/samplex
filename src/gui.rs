use iced::{Element, Sandbox, Text};

pub struct SamplexApp;

impl Sandbox for SamplexApp {
    type Message = ();

    fn new() -> SamplexApp {
        SamplexApp
    }

    fn title(&self) -> String {
        String::from("Samplex")
    }

    fn update(&mut self, _message: Self::Message) {
        // This application has no interactions
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello, world!").into()
    }
}
