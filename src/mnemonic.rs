use anyhow::Result;
// use rand::prelude::*;
use rand::rngs::OsRng;
use rand::TryRngCore;
use bip39::{Mnemonic};

/*
Need to use a Cryptographically Secure Random Number Generator to
generate 256 bits
*/
pub fn generate_entropy() -> Result<()> {
    let mut key = vec![0u8; 32];
    OsRng.try_fill_bytes(&mut key[..])?;
    let mnemonic = Mnemonic::from_entropy(&key)?;
    println!("Clé cryptographique (bytes): {:?}", mnemonic);
    for (i, word) in mnemonic.words().enumerate() {
        println!("{}. {}", i + 1, word);
    }
    Ok(())
}