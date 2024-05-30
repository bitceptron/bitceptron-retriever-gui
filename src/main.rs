use bitceptron_retriever_gui::RetrieverApp;
use iced::{font, Application, Pixels};
use tracing_log::LogTracer;

fn main() {
    LogTracer::init().unwrap();
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new()).unwrap();
    let settings = iced::Settings {
        default_font: iced::Font {
            family: font::Family::Monospace,
            ..Default::default()
        },
        default_text_size: Pixels::from(12),
        ..Default::default()
    };
    let _ = RetrieverApp::run(settings);
}
