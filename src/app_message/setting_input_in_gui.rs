
#[derive(Debug, Clone)]
pub enum SettingInputInGuiMessage {
    BitcoincoreUrlChanged(String),
    BitcoincoreRpcPortChanged(String),
    BitcoincoreTimeoutChanged(String),
    BitcoincoreCookiePathChanged(String),
    ExplorerNetworkChanged(String),
    MnemonicChanged(iced::widget::text_editor::Action),
}