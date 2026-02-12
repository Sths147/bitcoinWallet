use anyhow::Result;

mod btc_backend;
mod mnemonic;
mod electrum_backend;
use bip32::XPrv;
use mnemonic::{Wallet, generate_seed, get_account_xprv};
use btc_backend::BitcoinCoreRpc;
use electrum_backend::ElectrumBackend;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let core_client = BitcoinCoreRpc::new("http://localhost:18443", "user!".to_string(), "password!".to_string());
    let electrum_client= ElectrumBackend::new("electrum.blockstream.info").await?;
    // let info = client.client.list_unspent()?;
    // println!("{:?}", info);
    // println!("{:?}", check);
    let mut wallet = Wallet::new(get_account_xprv(generate_seed()?)?);
    wallet.recover(&electrum_client).await?;
    wallet.new_p2wpkh(1)?;
    wallet.new_p2wpkh(1)?;
    wallet.new_p2wpkh(1)?;
    wallet.new_p2wpkh(1)?;
    Ok(())
}

// fn main() -> Result<()> {
//     let rpc =  Client::new(
//         "http://localhost:18443",
//         Auth::UserPass("jfdkguiew".to_string(), "pEheOAHUQ00KUi0732ZLdibgMIIxU_sXPQIvhSOJXU4".to_string())
//     )?;
//     let block_count = rpc.get_block_count()?;
//     println!("Block count: {}", block_count);
//     let mining_infos = rpc.get_mining_info()?;
//     println!("Mining infos: {:?}", mining_infos);
//     let hash = rpc.get_best_block_hash()?;
//     let block = rpc.get_block(&hash)?;
//     println!("Best block: {:?}", block);
//     let balance = rpc.get_balance(None, None)?;
//     println!("Balance: {:?} BTC", balance.to_btc());
//     Ok(())
// }
