use anyhow::{Result, anyhow};
use bip39::{Mnemonic};
use bip32::{XPrv, DerivationPath, XPub};
use rand_core::{OsRng, RngCore};
use zeroize::{Zeroize, Zeroizing};
use std::collections::HashMap;

const GAP_LIMIT: u16 = 20;

/* 
For each address purpose, 2 pairs of index, derivation map are kept
derivation map to keep track of address/derivation paths matchings for spending
index to know the next derivation path to generate new address 
*/

#[derive(Default)]
struct AddressPurpose {
    path: String,
    internal_index: u32,
    external_index: u32,
    external_derivation_map: HashMap<String, String>,
    internal_derivation_map: HashMap<String, String>,
}

pub struct Wallet {
    xprv: XPrv,
    account: u8,
    bip84: AddressPurpose,
}

/*
use of OsRng to fulfill need to use a Cryptographically Secure Random 
Number Generator to generate 256 bits
Because OsRng can fail, we prefer to use the try_fill_bytes function
We call zeroize asap to avoid leaving entropy or mnemonic in memory, 
because clear or drop can be skipped by compiler for optimization
*/
pub fn generate_seed() -> Result<[u8; 64]> {
    let mut key = Zeroizing::new(vec![0u8; 32]);
    if let Err(e) = OsRng.try_fill_bytes(key.as_mut()) {
        return Err(anyhow!(e));
    }
    let mut mnemonic = match Mnemonic::from_entropy(key.as_ref()) {
        Ok(mnemonic) => mnemonic,
        Err(e) => {key.zeroize(); return Err(anyhow!(e))},
    };
    key.zeroize();
    println!("Clé cryptographique (bytes): {:?}", mnemonic);
    for (i, word) in mnemonic.words().enumerate() {
        println!("{}. {}", i + 1, word);
    }
    let seed = mnemonic.to_seed_normalized("mnemonic");
    mnemonic.zeroize();
    println!("seed: {:?}", seed);
    Ok(seed)
}

impl Wallet {
    pub fn new(xprv: XPrv) -> Self {
        Wallet {
            xprv,
            account: 0,
            bip84: AddressPurpose::default(),
        }
    }
    pub fn get_balance(&self) {

    }
}

/*
To do : Commencer par simple:
un seul account, uniquement BIP84
une seule structure
pour le recovery on fonctionne uniquement avec ce path la
*/

impl AddressPurpose {
    // get new address, je dois :
    /*
        Renvoyer le path
     */
    pub fn new(bip: u16, account: u16) -> Self {
        let path = format!("m/84'/0'/");
        AddressPurpose {
            path,
            ..Default::default()
        }
    }
    fn get_new_address(&mut self, situation: u8) {
        let index = if situation == 0 {&self.external_index} else {&self.external_index};
        
    }
    fn increase_index(&mut self, situation: u8) {
        if situation == 0 {
            if self.external_index == u32::MAX {
                eprintln!("Max addresses reached for external chain with {}", self.path);  
            }
            self.external_index += 1;
        } else {
            if self.internal_index == u32::MAX {
                eprintln!("Max addresses reached for external chain with {}", self.path);  
            }
            self.internal_index += 1;
        }
    }
}


pub fn derive_addresses() -> Result<()> {
    // let rootXprv = XPrv::new(&seed);
    let seed = generate_seed()?;
    
    let index = "0";
    // let xprv44 = XPrv::derive_from_path(&seed, &(BIP44PATH + index).parse()?)?;
    // let xprv49 = XPrv::derive_from_path(&seed, &(BIP49PATH + index).parse()?)?;
    // let xprv84 = XPrv::derive_from_path(&seed, &(BIP84PATH + index).parse()?)?;
    // let xprv86 = XPrv::derive_from_path(&seed, &(BIP86PATH + index).parse()?)?;
    // xprv44.zeroize();
    // println!("xprv44: {:?}", xprv44);
    // println!("xprv49: {:?}", xprv49);
    // println!("xprv84: {:?}", xprv84);
    // println!("xprv86: {:?}", xprv86);
    Ok(())
}
