use std::error::Error;

use bitceptron_retriever::{error::RetrieverError, setting::RetrieverSetting};

use crate::RetrieverApp;

pub fn create_retriever_setting(app: &mut RetrieverApp) -> Option<RetrieverSetting> {
    if app.bitcoincore_client_setting_input.is_input_fixed()
        && app.explorer_setting_input.is_input_fixed()
        && app.retriever_specific_setting_input.is_input_fixed()
    {
        Some(RetrieverSetting::new(
            Some(app.bitcoincore_client_setting_input.get_in_use_url()),
            Some(app.bitcoincore_client_setting_input.get_in_use_rpc_port()),
            app.bitcoincore_client_setting_input.get_in_use_cookie_path(),
            Some(app.bitcoincore_client_setting_input.get_in_use_timeout()),
            app.explorer_setting_input.get_in_use_mnemonic(),
            app.explorer_setting_input.get_in_use_passphrase(),
            Some(app.explorer_setting_input.get_in_use_base_derivation_paths()),
            Some(app.explorer_setting_input.get_in_use_exploration_path()),
            Some(app.retriever_specific_setting_input.get_in_use_selected_descriptors()),
            Some(app.explorer_setting_input.get_in_use_sweep()),
            Some(app.explorer_setting_input.get_in_use_exploration_depth()),
            Some(app.explorer_setting_input.get_in_use_network()),
            app.retriever_specific_setting_input.get_in_use_data_dir(),
        ))
    } else {
        None
    }
}
