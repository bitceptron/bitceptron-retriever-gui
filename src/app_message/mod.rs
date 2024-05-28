use std::sync::{Arc, Mutex};

use bitceptron_retriever::{error::RetrieverError, retriever::Retriever};

use self::{setting_input_fixed::SettingInputFixedMessage, setting_input_in_gui::SettingInputInGuiMessage};

pub mod setting_input_in_gui;
pub mod setting_input_fixed;

#[derive(Debug, Clone)]
pub enum AppMessage {
    SettingInputInGuiChanged(SettingInputInGuiMessage),
    SettingInputGotFixed(SettingInputFixedMessage),
    CreateRetriever,
    PrepareDumpFile,
    DumpFileDone,
    PopulateUtxoDB,
    Search,
    RetrieverCreated(Retriever),
    Error(Arc<RetrieverError>),
    None,
}