use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use bitceptron_retriever::error::RetrieverError;
use num_format::{Locale, ToFormattedString};
use tokio_util::sync::CancellationToken;
use tracing::info;


#[derive(Debug, Clone)]
pub struct UnspentScriptPubKeysSet {
    set: Arc<hashbrown::HashSet<Vec<u8>>>,
    status: Arc<Mutex<Vec<UspkSetStatus>>>,
}

impl Default for UnspentScriptPubKeysSet {
    fn default() -> Self {
        Self::new()
    }
}

impl UnspentScriptPubKeysSet {
    pub fn new() -> Self {
        let set: hashbrown::HashSet<Vec<u8>> = hashbrown::HashSet::new();
        UnspentScriptPubKeysSet {
            set: Arc::new(set),
            status: Arc::new(Mutex::new(vec![UspkSetStatus::Empty])),
        }
    }
    pub async fn populate_with_dump_file(
        &mut self,
        dump_file_path: &str,
        cancellation_token: CancellationToken,
    ) -> Result<(), RetrieverError> {
        let creation_start = Instant::now();
        let status = self.status.clone();
        let (set_sender, set_receiver) = tokio::sync::oneshot::channel();
        let mut dump = txoutset::Dump::new(dump_file_path, txoutset::ComputeAddresses::No)?;
        // Loop information.
        let step_size = 100000u64;
        let mut average_step_time_in_micros = 0u128;
        let total_loops = dump.utxo_set_size;
        let mut loops_done = 0u64;
        let mut steps_done = 0u128;
        let mut steps_remaining = (total_loops / step_size) as u128;
        let mut step_start_time = Instant::now();
        // Loop.
        tokio::task::spawn_blocking(move || {
            status.lock().unwrap()[0] = UspkSetStatus::Populating;
            let mut set = hashbrown::HashSet::new();
            while !cancellation_token.is_cancelled() {
                match dump.next() {
                    Some(txout) => {
                        set.insert(txout.script_pubkey.as_bytes().to_vec());
                        // Loop info stuff.
                        loops_done += 1;
                        if loops_done % step_size == 0 {
                            steps_done += 1;
                            steps_remaining -= 1;
                            average_step_time_in_micros = (step_start_time.elapsed().as_micros()
                                + (steps_done - 1) * average_step_time_in_micros)
                                / steps_done;
                            let remaining_time_in_milis =
                                average_step_time_in_micros * steps_remaining;
                            info!(
                                "Utxos moved to database: {} of {}",
                                loops_done.to_formatted_string(&Locale::en),
                                total_loops.to_formatted_string(&Locale::en)
                            );
                            info!(
                                "Estimated time to completion: ~{} minutes.",
                                (1 + remaining_time_in_milis / 60_000_000)
                                    .to_formatted_string(&Locale::en)
                            );
                            step_start_time = Instant::now();
                        }
                    }
                    None => {
                        let _ = set_sender.send(set);
                        status.lock().unwrap()[0] = UspkSetStatus::Ready;
                        break;
                    }
                }
            }
        });
        info!(
            "UTXO database of {} unspent scripts populated in ~{} mins.",
            total_loops.to_formatted_string(&Locale::en),
            1 + creation_start.elapsed().as_secs() / 60
        );
        self.set = Arc::new(set_receiver.await.unwrap());
        Ok(())
    }

    pub fn get_immutable_inner_set(&self) -> Arc<hashbrown::HashSet<Vec<u8>>> {
        self.set.clone()
    }

    pub fn get_status(&self) -> UspkSetStatus {
        self.status.lock().unwrap()[0]
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UspkSetStatus {
    Empty,
    Populating,
    Ready,
}
