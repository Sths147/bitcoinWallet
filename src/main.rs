use anyhow::Result;

mod btc_backend;
mod mnemonic;
mod electrum_backend;
use mnemonic::{Wallet, generate_seed, get_account_xprv};
// use btc_backend::BitcoinCoreRpc;
use electrum_backend::ElectrumBackend;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // let core_client = BitcoinCoreRpc::new("http://localhost:18443", "".to_string(), "".to_string());
    let electrum_client= ElectrumBackend::new("electrum.blockstream.info").await?;
    let mut wallet = Wallet::new(get_account_xprv(generate_seed()?)?);
    wallet.recover(&electrum_client).await?;
    wallet.new_p2wpkh(1)?;
    Ok(())
}
