use anyhow::{Result};

mod mnemonic;
use mnemonic::{Wallet, generate_seed};


fn main() -> Result<()> {
    let wallet = Wallet::new(generate_seed()?);
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
