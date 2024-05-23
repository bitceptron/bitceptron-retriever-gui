use self::setting_update_message::SettingUpdateMessage;

pub mod setting_update_message;

#[derive(Debug, Clone)]
pub enum AppMessage {
    SettingUpdate(SettingUpdateMessage)
}