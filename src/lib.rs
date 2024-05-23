use app_message::AppMessage;
use iced::{executor, Application, Command};
use view_elements::bitcoincore_client_setting_row;

pub mod app_message;
pub mod status;
pub mod view_elements;
pub mod inputs;
pub mod gui_error;

pub struct RetrieverApp {
}

impl Application for RetrieverApp {
    type Executor = executor::Default;

    type Message = AppMessage;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (RetrieverApp{}, Command::none())
    }

    fn title(&self) -> String {
        String::from("bitceptron retriever")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        todo!()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        bitcoincore_client_setting_row(self)
    }
}