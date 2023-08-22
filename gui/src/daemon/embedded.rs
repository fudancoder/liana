use std::collections::HashMap;

use super::{model::*, Daemon, DaemonError};
use liana::{
    config::Config,
    miniscript::bitcoin::{address, psbt::Psbt, Address, OutPoint, Txid},
    DaemonHandle,
};

pub struct EmbeddedDaemon {
    config: Config,
    handle: DaemonHandle,
}

impl EmbeddedDaemon {
    pub fn start(config: Config) -> Result<EmbeddedDaemon, DaemonError> {
        let handle = DaemonHandle::start_default(config.clone()).map_err(DaemonError::Start)?;
        Ok(Self { handle, config })
    }
}

impl std::fmt::Debug for EmbeddedDaemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DaemonHandle").finish()
    }
}

impl Daemon for EmbeddedDaemon {
    fn is_external(&self) -> bool {
        false
    }

    fn config(&self) -> Option<&Config> {
        Some(&self.config)
    }

    fn stop(&self) {
        self.handle.trigger_shutdown();
        while !self.handle.shutdown_complete() {
            tracing::debug!("Waiting daemon to shutdown");
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }

    fn get_info(&self) -> Result<GetInfoResult, DaemonError> {
        Ok(self.handle.control.get_info())
    }

    fn get_new_address(&self) -> Result<GetAddressResult, DaemonError> {
        Ok(self.handle.control.get_new_address())
    }

    fn list_coins(&self) -> Result<ListCoinsResult, DaemonError> {
        Ok(self.handle.control.list_coins())
    }

    fn list_spend_txs(&self) -> Result<ListSpendResult, DaemonError> {
        Ok(self.handle.control.list_spend())
    }

    fn list_confirmed_txs(
        &self,
        start: u32,
        end: u32,
        limit: u64,
    ) -> Result<ListTransactionsResult, DaemonError> {
        Ok(self
            .handle
            .control
            .list_confirmed_transactions(start, end, limit))
    }

    fn list_txs(&self, txids: &[Txid]) -> Result<ListTransactionsResult, DaemonError> {
        Ok(self.handle.control.list_transactions(txids))
    }

    fn create_spend_tx(
        &self,
        coins_outpoints: &[OutPoint],
        destinations: &HashMap<Address<address::NetworkUnchecked>, u64>,
        feerate_vb: u64,
    ) -> Result<CreateSpendResult, DaemonError> {
        self.handle
            .control
            .create_spend(destinations, coins_outpoints, feerate_vb)
            .map_err(|e| DaemonError::Unexpected(e.to_string()))
    }

    fn update_spend_tx(&self, psbt: &Psbt) -> Result<(), DaemonError> {
        self.handle
            .control
            .update_spend(psbt.clone())
            .map_err(|e| DaemonError::Unexpected(e.to_string()))
    }

    fn delete_spend_tx(&self, txid: &Txid) -> Result<(), DaemonError> {
        self.handle.control.delete_spend(txid);
        Ok(())
    }

    fn broadcast_spend_tx(&self, txid: &Txid) -> Result<(), DaemonError> {
        self.handle
            .control
            .broadcast_spend(txid)
            .map_err(|e| DaemonError::Unexpected(e.to_string()))
    }

    fn start_rescan(&self, t: u32) -> Result<(), DaemonError> {
        self.handle
            .control
            .start_rescan(t)
            .map_err(|e| DaemonError::Unexpected(e.to_string()))
    }

    fn create_recovery(
        &self,
        address: Address<address::NetworkUnchecked>,
        feerate_vb: u64,
        sequence: Option<u16>,
    ) -> Result<Psbt, DaemonError> {
        self.handle
            .control
            .create_recovery(address, feerate_vb, sequence)
            .map_err(|e| DaemonError::Unexpected(e.to_string()))
            .map(|res| res.psbt)
    }
}
