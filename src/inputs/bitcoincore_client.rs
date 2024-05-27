use std::{path::PathBuf, str::FromStr};

use getset::Getters;
use regex::Regex;

use crate::gui_error::GuiError;

use super::gui_input::GuiInput;

#[derive(Debug)]
pub struct BitcoincoreClientInput {
    gui_input: BitcoincoreClientSettingFromGui,
    in_use: Option<BitcoincoreClientSettingInUse>,
}

impl Default for BitcoincoreClientInput {
    fn default() -> Self {
        Self::new()
    }
}

impl BitcoincoreClientInput {
    pub fn new() -> Self {
        BitcoincoreClientInput {
            gui_input: BitcoincoreClientSettingFromGui::default(),
            in_use: None,
        }
    }

    pub fn gui_to_in_use(&mut self) -> Result<(), GuiError> {
        let in_use = match self.is_gui_input_sane() {
            true => BitcoincoreClientSettingInUse {
                in_use_url: self.get_gui_url(),
                in_use_rpc_port: self.get_gui_rpc_port(),
                in_use_timeout_seconds: self.get_gui_timeout().parse::<u64>().unwrap(),
                in_use_cookie_path: self.get_gui_cookie_path(),
            },
            false => return Err(GuiError::GuiInputIsInsane),
        };
        self.in_use = Some(in_use);
        Ok(())
    }

    pub fn set_url_from_gui_input(&mut self, url: String) {
        self.gui_input.gui_url = UrlGuiData::new(url.trim().to_string());
    }

    pub fn set_rpc_port_from_gui_input(&mut self, rpc_port: String) {
        self.gui_input.gui_rpc_port = RpcPortGuiData::new(rpc_port.trim().to_string());
    }

    pub fn set_timeout_from_gui_input(&mut self, timeout: String) {
        self.gui_input.gui_timeout = TimeoutGuiData::new(timeout.trim().to_string());
    }

    pub fn set_cookie_path_from_gui_input(&mut self, cookie_path: String) {
        self.gui_input.gui_cookie_path = CookiePathGuiData::new(cookie_path.trim().to_string());
    }

    pub fn get_gui_url(&self) -> String {
        self.gui_input.gui_url.get_value()
    }

    pub fn get_gui_rpc_port(&self) -> String {
        self.gui_input.gui_rpc_port.get_value()
    }

    pub fn get_gui_timeout(&self) -> String {
        self.gui_input.gui_timeout.get_value()
    }

    pub fn get_gui_cookie_path(&self) -> String {
        self.gui_input.gui_cookie_path.get_value()
    }

    pub fn get_in_use_url(&self) -> String {
        match &self.in_use {
            Some(in_use) => in_use.get_in_use_url().to_owned(),
            None => "".to_owned(),
        }
    }

    pub fn get_in_use_rpc_port(&self) -> String {
        match &self.in_use {
            Some(in_use) => in_use.get_in_use_rpc_port().to_owned(),
            None => "".to_owned(),
        }
    }

    pub fn get_in_use_timeout(&self) -> String {
        match &self.in_use {
            Some(in_use) => in_use.get_in_use_timeout_seconds().to_string(),
            None => "".to_owned(),
        }
    }

    pub fn get_in_use_cookie_path(&self) -> String {
        match &self.in_use {
            Some(in_use) => in_use.get_in_use_cookie_path().to_owned(),
            None => "".to_owned(),
        }
    }

    pub fn is_gui_input_sane(&self) -> bool {
        self.gui_input.gui_url.is_sane()
            && self.gui_input.gui_rpc_port.is_sane()
            && self.gui_input.gui_timeout.is_sane()
            && self.gui_input.gui_cookie_path.is_sane()
    }

    pub fn is_gui_url_sane(&self) -> bool {
        self.gui_input.gui_url.sanity
    }

    pub fn is_gui_rpc_port_sane(&self) -> bool {
        self.gui_input.gui_rpc_port.sanity
    }

    pub fn is_gui_timeout_sane(&self) -> bool {
        self.gui_input.gui_timeout.sanity
    }

    pub fn is_gui_cookie_path_sane(&self) -> bool {
        self.gui_input.gui_cookie_path.sanity
    }

    pub fn is_url_fixed(&self) -> bool {
        self.in_use.is_some() && (self.get_gui_url() == self.get_in_use_url())
    }

    pub fn is_rpc_port_fixed(&self) -> bool {
        self.in_use.is_some() && (self.get_gui_rpc_port() == self.get_in_use_rpc_port())
    }

    pub fn is_timeout_fixed(&self) -> bool {
        self.in_use.is_some() && (self.get_gui_timeout() == self.get_in_use_timeout())
    }

    pub fn is_cookie_path_fixed(&self) -> bool {
        self.in_use.is_some() && (self.get_gui_cookie_path() == self.get_in_use_cookie_path())
    }

    pub fn is_input_fixed(&self) -> bool {
        self.get_gui_url() == self.get_in_use_url()
            && self.get_gui_rpc_port() == self.get_in_use_rpc_port()
            && self.get_gui_timeout() == self.get_in_use_timeout()
            && self.get_gui_cookie_path() == self.get_in_use_cookie_path()
    }
}

#[derive(Debug, Getters)]
#[get = "pub with_prefix"]
pub struct BitcoincoreClientSettingFromGui {
    gui_url: UrlGuiData,
    gui_rpc_port: RpcPortGuiData,
    gui_timeout: TimeoutGuiData,
    gui_cookie_path: CookiePathGuiData,
}

impl Default for BitcoincoreClientSettingFromGui {
    fn default() -> Self {
        Self {
            gui_url: UrlGuiData::new(
                bitceptron_retriever::data::defaults::DEFAULT_BITCOINCORE_RPC_URL.to_owned(),
            ),
            gui_rpc_port: RpcPortGuiData::new(
                bitceptron_retriever::data::defaults::DEFAULT_BITCOINCORE_RPC_PORT.to_owned(),
            ),
            gui_timeout: TimeoutGuiData::new(
                bitceptron_retriever::data::defaults::DEFAULT_BITCOINCORE_RPC_TIMEOUT_SECONDS
                    .to_string()
                    .to_owned(),
            ),
            gui_cookie_path: CookiePathGuiData::new(String::from("")),
        }
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
        UrlGuiData {
            url: url.trim().to_string(),
            sanity,
        }
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

#[derive(Debug, Getters)]
#[get = "pub with_prefix"]
pub struct BitcoincoreClientSettingInUse {
    in_use_url: String,
    in_use_rpc_port: String,
    in_use_timeout_seconds: u64,
    in_use_cookie_path: String,
}
