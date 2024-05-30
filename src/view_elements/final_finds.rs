use std::fmt::{Debug, Display};

use bitceptron_retriever::path_pairs::PathScanResultDescriptorTrio;
use bitcoin::{bip32::DerivationPath, secp256k1::PublicKey};
use miniscript::Descriptor;
use num_format::{Locale, ToFormattedString};

pub struct FinalFinds {
    result_num: u64,
    path: DerivationPath,
    amount_in_sat: u64,
    descriptor: Descriptor<PublicKey>,
}

impl FinalFinds {
    pub fn new(result_num: u64, details: &PathScanResultDescriptorTrio) -> Self {
        FinalFinds {
            result_num,
            path: details.0.clone(),
            amount_in_sat: details.1.total_amount.to_sat(),
            descriptor: details.2.clone(),
        }
    }
}

impl Display for FinalFinds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nResult {}\nPath: {}\nAmount(satoshis): {}\nDescriptor: {}\n",
            self.result_num, self.path, self.amount_in_sat.to_formatted_string(&Locale::en), self.descriptor
        )
    }
}

impl Debug for FinalFinds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
