
#[derive(Debug, Clone)]
pub enum SettingInputInGuiMessage {
    BitcoincoreUrlChanged(String),
    BitcoincoreRpcPortChanged(String),
    BitcoincoreTimeoutChanged(String),
    BitcoincoreCookiePathChanged(String),
    BaseDerivationPathsChanged(String),
    BaseDerivationPathsFromPresetsChanged(bool),
    ExplorationPathChanged(String),
    SweepChanged(bool),
    ExplorationDepthChanged(String),
    NetworkChanged(bitcoin::Network),
    P2pkInclusionChanged(bool),
    P2pkhInclusionChanged(bool),
    P2wpkhInclusionChanged(bool),
    P2shwpkhInclusionChanged(bool),
    P2trInclusionChanged(bool),
    DataDirChanged(String),
    MnemonicChanged(iced::widget::text_editor::Action),
    PassphraseChanged(String),
}