use std::{path::PathBuf, str::FromStr};

use bitceptron_retriever::covered_descriptors::CoveredDescriptors;

use crate::gui_error::GuiError;

use super::gui_input::GuiInput;

#[derive(Debug)]
pub struct RetrieverSpecificInput {
    gui_input: RetrieverSpecificSettingFromGui,
    in_use: Option<RetrieverSpecificSettingInUse>,
}

impl Default for RetrieverSpecificInput {
    fn default() -> Self {
        Self {
            gui_input: Default::default(),
            in_use: None,
        }
    }
}

impl RetrieverSpecificInput {
    pub fn new() -> Self {
        RetrieverSpecificInput::default()
    }

    pub fn gui_to_in_use(&mut self) -> Result<(), GuiError> {
        let in_use = match self.is_gui_input_sane() {
            true => RetrieverSpecificSettingInUse {
                in_use_selected_descriptors: {
                    let mut res = vec![];
                    if self.get_gui_p2pk() {
                        res.push(CoveredDescriptors::P2pk)
                    };
                    if self.get_gui_p2pkh() {
                        res.push(CoveredDescriptors::P2pkh)
                    };

                    if self.get_gui_p2wpkh() {
                        res.push(CoveredDescriptors::P2wpkh)
                    };

                    if self.get_gui_p2shwpkh() {
                        res.push(CoveredDescriptors::P2shwpkh)
                    };

                    if self.get_gui_p2tr() {
                        res.push(CoveredDescriptors::P2tr)
                    };
                    res
                },
                in_use_data_dir: self.get_gui_data_dir(),
            },
            false => return Err(GuiError::GuiInputIsInsane),
        };
        self.in_use = Some(in_use);
        Ok(())
    }

    pub fn set_p2pk_inclusion_from_gui_input(&mut self, p2pk_inclusion: bool) {
        self.gui_input.gui_p2pk = p2pk_inclusion
    }

    pub fn set_p2pkh_inclusion_from_gui_input(&mut self, p2pkh_inclusion: bool) {
        self.gui_input.gui_p2pkh = p2pkh_inclusion
    }

    pub fn set_p2wpkh_inclusion_from_gui_input(&mut self, p2wpkh_inclusion: bool) {
        self.gui_input.gui_p2wpkh = p2wpkh_inclusion
    }

    pub fn set_p2shwpkh_inclusion_from_gui_input(&mut self, p2shwpkh_inclusion: bool) {
        self.gui_input.gui_p2shwpkh = p2shwpkh_inclusion
    }

    pub fn set_p2tr_inclusion_from_gui_input(&mut self, p2tr_inclusion: bool) {
        self.gui_input.gui_p2tr = p2tr_inclusion
    }

    pub fn set_data_dir_from_gui_input(&mut self, data_dir: String) {
        self.gui_input.gui_data_dir = DataDirGuiData::new(data_dir)
    }

    pub fn get_gui_p2pk(&self) -> bool {
        self.gui_input.gui_p2pk
    }

    pub fn get_gui_p2pkh(&self) -> bool {
        self.gui_input.gui_p2pkh
    }

    pub fn get_gui_p2wpkh(&self) -> bool {
        self.gui_input.gui_p2wpkh
    }

    pub fn get_gui_p2shwpkh(&self) -> bool {
        self.gui_input.gui_p2shwpkh
    }

    pub fn get_gui_p2tr(&self) -> bool {
        self.gui_input.gui_p2tr
    }

    pub fn get_gui_data_dir(&self) -> String {
        self.gui_input.gui_data_dir.get_value()
    }

    pub fn get_in_use_p2pk(&self) -> bool {
        match &self.in_use {
            Some(in_use) => in_use
                .in_use_selected_descriptors
                .contains(&CoveredDescriptors::P2pk),
            None => false,
        }
    }

    pub fn get_in_use_p2pkh(&self) -> bool {
        match &self.in_use {
            Some(in_use) => in_use
                .in_use_selected_descriptors
                .contains(&CoveredDescriptors::P2pkh),
            None => false,
        }
    }

    pub fn get_in_use_p2wpkh(&self) -> bool {
        match &self.in_use {
            Some(in_use) => in_use
                .in_use_selected_descriptors
                .contains(&CoveredDescriptors::P2wpkh),
            None => false,
        }
    }

    pub fn get_in_use_p2shwpkh(&self) -> bool {
        match &self.in_use {
            Some(in_use) => in_use
                .in_use_selected_descriptors
                .contains(&CoveredDescriptors::P2shwpkh),
            None => false,
        }
    }

    pub fn get_in_use_p2tr(&self) -> bool {
        match &self.in_use {
            Some(in_use) => in_use
                .in_use_selected_descriptors
                .contains(&CoveredDescriptors::P2tr),
            None => false,
        }
    }

    pub fn get_in_use_selected_descriptors(&self) -> Vec<CoveredDescriptors> {
        match &self.in_use {
            Some(in_use) => in_use.in_use_selected_descriptors.clone(),
            None => vec![],
        }
    }

    pub fn get_in_use_data_dir(&self) -> String {
        match &self.in_use {
            Some(in_use) => in_use.in_use_data_dir.to_string(),
            None => "".to_string(),
        }
    }

    pub fn is_gui_selected_descriptors_sane(&self) -> bool {
        self.get_gui_p2pk()
            || self.get_gui_p2pkh()
            || self.get_gui_p2wpkh()
            || self.get_gui_p2shwpkh()
            || self.get_gui_p2tr()
    }

    pub fn is_gui_data_dir_sane(&self) -> bool {
        self.gui_input.gui_data_dir.is_sane()
    }

    pub fn is_gui_input_sane(&self) -> bool {
        self.is_gui_data_dir_sane() && self.is_gui_selected_descriptors_sane()
    }

    pub fn is_data_dir_fixed(&self) -> bool {
        self.in_use.is_some() && (self.get_gui_data_dir() == self.get_in_use_data_dir())
    }

    pub fn is_selected_descriptors_fixed(&self) -> bool {
        self.in_use.is_some()
            && self.get_gui_p2pk() == self.get_in_use_p2pk()
            && self.get_gui_p2pkh() == self.get_in_use_p2pkh()
            && self.get_gui_p2wpkh() == self.get_in_use_p2wpkh()
            && self.get_gui_p2shwpkh() == self.get_in_use_p2shwpkh()
            && self.get_gui_p2tr() == self.get_in_use_p2tr()
    }

    pub fn is_input_fixed(&self) -> bool {
        self.is_data_dir_fixed() && self.is_selected_descriptors_fixed()
    }
}

#[derive(Debug)]
pub struct RetrieverSpecificSettingFromGui {
    gui_p2pk: bool,
    gui_p2pkh: bool,
    gui_p2wpkh: bool,
    gui_p2shwpkh: bool,
    gui_p2tr: bool,
    gui_data_dir: DataDirGuiData,
}

impl Default for RetrieverSpecificSettingFromGui {
    fn default() -> Self {
        Self {
            gui_p2pk: true,
            gui_p2pkh: true,
            gui_p2wpkh: true,
            gui_p2shwpkh: true,
            gui_p2tr: true,
            gui_data_dir: DataDirGuiData::new("".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct DataDirGuiData {
    data_dir: String,
    sanity: bool,
}

impl GuiInput for DataDirGuiData {
    fn new(value: String) -> Self {
        let data_dir = value.trim().to_string();
        let path = PathBuf::from_str(&data_dir).unwrap();
        let sanity = path.canonicalize().is_ok()
            && path.canonicalize().unwrap().exists()
            && path.canonicalize().unwrap().is_dir();
        DataDirGuiData { data_dir, sanity }
    }

    fn is_sane(&self) -> bool {
        self.sanity
    }

    fn get_value(&self) -> String {
        self.data_dir.to_owned()
    }
}

#[derive(Debug)]
pub struct RetrieverSpecificSettingInUse {
    in_use_selected_descriptors: Vec<CoveredDescriptors>,
    in_use_data_dir: String,
}
