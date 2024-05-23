use std::{path::PathBuf, str::FromStr};

use getset::Getters;
use regex::Regex;

use crate::gui_error::GuiError;

use super::gui_input::GuiInput;

pub struct BitcoincoreClientInput {
    gui_input: BitcoincoreClientSettingFromGui,
    in_use: Option<BitcoincoreClientSettingInUse>,
}

impl BitcoincoreClientInput {
    pub fn new() -> Self {
        BitcoincoreClientInput {
            gui_input: BitcoincoreClientSettingFromGui::default(),
            in_use: None,
        }
    }

    pub fn gui_to_in_use(&mut self) -> Result<(), GuiError> {
        let in_use = match BitcoincoreClientSettingInUse::from_gui_input(&self.gui_input) {
            Ok(input) => Some(input),
            Err(e) => return Err(e),
        };
        self.in_use = in_use;
        Ok(())
    }
}

#[derive(Debug, Getters)]
#[get = "pub with_prefix"]
pub struct BitcoincoreClientSettingFromGui {
    url: UrlGuiData,
    rpc_port: RpcPortGuiData,
    timeout: TimeoutGuiData,
    cookie_path: CookiePathGuiData,
}

impl Default for BitcoincoreClientSettingFromGui {
    fn default() -> Self {
        Self {
            url: UrlGuiData::new(
                bitceptron_retriever::data::defaults::DEFAULT_BITCOINCORE_RPC_URL.to_owned(),
            ),
            rpc_port: RpcPortGuiData::new(
                bitceptron_retriever::data::defaults::DEFAULT_BITCOINCORE_RPC_PORT.to_owned(),
            ),
            timeout: TimeoutGuiData::new(
                bitceptron_retriever::data::defaults::DEFAULT_BITCOINCORE_RPC_TIMEOUT_SECONDS
                    .to_string()
                    .to_owned(),
            ),
            cookie_path: CookiePathGuiData::new(String::from("Enter `.cookie` path.")),
        }
    }
}

impl BitcoincoreClientSettingFromGui {
    pub fn set_url(&mut self, url: String) {
        self.url = UrlGuiData::new(url);
    }

    pub fn set_rpc_port(&mut self, rpc_port: String) {
        self.rpc_port = RpcPortGuiData::new(rpc_port);
    }

    pub fn set_timeout(&mut self, timeout: String) {
        self.timeout = TimeoutGuiData::new(timeout);
    }

    pub fn set_cookie_path(&mut self, cookie_path: String) {
        self.cookie_path = CookiePathGuiData::new(cookie_path);
    }

    pub fn is_sane(&self) -> bool {
        self.url.is_sane()
            && self.rpc_port.is_sane()
            && self.timeout.is_sane()
            && self.cookie_path.is_sane()
    }
}

#[derive(Debug)]
pub struct UrlGuiData {
    url: String,
    sanity: bool,
}

impl GuiInput for UrlGuiData {
    fn new(url: String) -> Self {
        let url_regex = Regex::new(r"^\d+.\d+.\d+.\d+$").unwrap();
        let sanity = url_regex.is_match(&url);
        UrlGuiData { url, sanity }
    }

    fn is_sane(&self) -> bool {
        self.sanity
    }

    fn get_value(&self) -> String {
        self.url.to_owned()
    }
}

#[derive(Debug)]
pub struct RpcPortGuiData {
    rpc_port: String,
    sanity: bool,
}

impl GuiInput for RpcPortGuiData {
    fn new(rpc_port: String) -> Self {
        let rpc_port_regex = Regex::new(r"^\d+$").unwrap();
        let sanity = rpc_port_regex.is_match(&rpc_port);
        RpcPortGuiData { rpc_port, sanity }
    }

    fn is_sane(&self) -> bool {
        self.sanity
    }

    fn get_value(&self) -> String {
        self.rpc_port.to_owned()
    }
}

#[derive(Debug)]
pub struct TimeoutGuiData {
    timeout: String,
    sanity: bool,
}

impl GuiInput for TimeoutGuiData {
    fn new(timeout: String) -> Self {
        let timeout_regex = Regex::new(r"^\d+$").unwrap();
        let sanity = timeout_regex.is_match(&timeout) && timeout.parse::<u64>().is_ok();
        TimeoutGuiData { timeout, sanity }
    }

    fn is_sane(&self) -> bool {
        self.sanity
    }

    fn get_value(&self) -> String {
        self.timeout.to_owned()
    }
}

#[derive(Debug)]
pub struct CookiePathGuiData {
    cookie_path: String,
    sanity: bool,
}

impl GuiInput for CookiePathGuiData {
    fn new(cookie_path: String) -> Self {
        let sanity = PathBuf::from_str(&cookie_path).is_ok()
            && PathBuf::from_str(&cookie_path).unwrap().exists()
            && PathBuf::from_str(&cookie_path).unwrap().is_file()
            && PathBuf::from_str(&cookie_path)
                .unwrap()
                .ends_with(".cookie");
        CookiePathGuiData {
            cookie_path,
            sanity,
        }
    }

    fn is_sane(&self) -> bool {
        self.sanity
    }

    fn get_value(&self) -> String {
        self.cookie_path.to_owned()
    }
}

pub struct BitcoincoreClientSettingInUse {
    rpc_url: String,
    rpc_port: String,
    cookie_path: String,
    timeout_seconds: u64,
}

impl BitcoincoreClientSettingInUse {
    pub fn from_gui_input(gui_input: &BitcoincoreClientSettingFromGui) -> Result<Self, GuiError> {
        match gui_input.is_sane() {
            true => Ok(BitcoincoreClientSettingInUse {
                rpc_url: gui_input.url.get_value(),
                rpc_port: gui_input.rpc_port.get_value(),
                cookie_path: gui_input.cookie_path.get_value(),
                timeout_seconds: gui_input.timeout.get_value().parse().unwrap(),
            }),
            false => Err(GuiError::GuiInputIsInsane),
        }
    }
}
