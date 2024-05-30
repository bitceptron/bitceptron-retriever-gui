use std::sync::Arc;

use bitceptron_retriever::{
    client::BitcoincoreRpcClient,
    error::RetrieverError,
    path_pairs::{PathDescriptorPair, PathScanResultDescriptorTrio},
};

use crate::uspk_set::UnspentScriptPubKeysSet;

use self::{
    setting_input_fixed::SettingInputFixedMessage, setting_input_in_gui::SettingInputInGuiMessage,
};

pub mod setting_input_fixed;
pub mod setting_input_in_gui;

#[derive(Debug, Clone)]
pub enum AppMessage {
    SettingInputInGuiChanged(SettingInputInGuiMessage),
    SettingInputGotFixed(SettingInputFixedMessage),
    // CreateExplorer,
    CreateClientForDumpFileAndThenPrepare,
    ClientCreatedForDumpFileSoPrepareDumpFile(BitcoincoreRpcClient),
    CreateClientForNewDumpFileAndThenCreate,
    ClientCreatedForNewFileSoCreateDumpFile(BitcoincoreRpcClient),
    DumpFilePrepared,
    PopulateUtxoDB,
    StopPopulatingUtxoDB,
    Search,
    StopSearch,
    SearchResultPrepared(Vec<PathDescriptorPair>),
    // RetrieverCreated(Retriever),
    SetPopulated(UnspentScriptPubKeysSet),
    CreateClientForGettingDetailsAndThenGet,
    ClientCreatedForGettingDetailsSoGetDetails(BitcoincoreRpcClient),
    DetailsReady(Option<Vec<PathScanResultDescriptorTrio>>),
    Error(Arc<RetrieverError>),
    None,
}
