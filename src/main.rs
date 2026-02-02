use anyhow::{Result};
use bitcoincore_rpc::{Auth, Client, RpcApi};

fn main() -> Result<()> {
    let rpc =  Client::new(
        "http://localhost:48332",
        Auth::UserPass("".to_string(), "".to_string())
    )?;
    let block_count = rpc.get_block_count().unwrap();
    println!("Block count: {}", block_count);
    let mining_infos = rpc.get_mining_info()?;
    println!("Mining infos: {:?}", mining_infos);
    let hash = rpc.get_best_block_hash()?;
    let block = rpc.get_block(&hash)?;
    println!("Best block: {:?}", block);
    let balance = rpc.get_balance(None, None)?;
    println!("Balance: {:?} BTC", balance.to_btc());
    Ok(())
}
