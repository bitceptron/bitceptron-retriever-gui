use std::sync::{Arc, Mutex};

use bitceptron_retriever::{
    client::BitcoincoreRpcClient,
    error::RetrieverError,
    path_pairs::{PathDescriptorPair, PathScanResultDescriptorTrio},
    retriever::Retriever,
    uspk_set::UnspentScriptPupKeysSet,
};

use self::{
    setting_input_fixed::SettingInputFixedMessage, setting_input_in_gui::SettingInputInGuiMessage,
};

pub mod setting_input_fixed;
pub mod setting_input_in_gui;

#[derive(Debug, Clone)]
pub enum AppMessage {
    SettingInputInGuiChanged(SettingInputInGuiMessage),
    SettingInputGotFixed(SettingInputFixedMessage),
    CreateRetriever,
    CreateClientForDumpFile,
    ClientCreatedForDumpFileSoPrepareDumpFile(BitcoincoreRpcClient),
    DumpFilePrepared,
    PopulateUtxoDB,
    Search,
    SearchResultPrepared(Vec<PathDescriptorPair>),
    RetrieverCreated(Retriever),
    SetPopulated(UnspentScriptPupKeysSet),
    Error(Arc<RetrieverError>),
    GetDetails,
    DetailsReady(Option<Vec<PathScanResultDescriptorTrio>>),
    None,
}
