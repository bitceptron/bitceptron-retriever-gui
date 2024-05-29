use std::sync::{Arc, Mutex};

use bitceptron_retriever::{
    client::BitcoincoreRpcClient,
    error::RetrieverError,
    path_pairs::{PathDescriptorPair, PathScanResultDescriptorTrio},
    retriever::Retriever,
    uspk_set::UnspentScriptPubKeysSet,
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
    CreateClientForDumpFileAndThenPrepare,
    ClientCreatedForDumpFileSoPrepareDumpFile(BitcoincoreRpcClient),
    DumpFilePrepared,
    PopulateUtxoDB,
    Search,
    SearchResultPrepared(Vec<PathDescriptorPair>),
    RetrieverCreated(Retriever),
    SetPopulated(UnspentScriptPubKeysSet),
    CreateClientForGettingDetailsAndThenGet,
    ClientCreatedForGettingDetailsSoGetDetails(BitcoincoreRpcClient),
    DetailsReady(Option<Vec<PathScanResultDescriptorTrio>>),
    Error(Arc<RetrieverError>),
    None,
}
