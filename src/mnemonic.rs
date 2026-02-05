use anyhow::{Result, anyhow};
use bip39::{Mnemonic};
use bip32::{XPrv, XPub};
use rand_core::{OsRng, RngCore};
use zeroize::{Zeroize, Zeroizing};
use std::collections::HashMap;

const GAP_LIMIT: u16 = 20;
const bip32path: &str = "m/32'/0'/0/0/";
const bip44path: &str = "m/44'/0'/0/0/";
const bip49path: &str = "m/49'/0'/0/0/";
const bip84path: &str = "m/84'/0'/0/0/";
const bip86path: &str = "m/86'/0'/0/0/";

enum AddressLocation {
    EXTERNAL,
    INTERNAL,
}

/* 
For each address purpose, 2 pairs of index, derivation map are kept
derivation map to keep track of address/derivation paths matchings for spending
index to know the next derivation path to generate new address 
*/

#[derive(Default)]
struct AddressPurpose {
    internal_index: u16,
    external_index: u16,
    external_derivation_map: HashMap<String, String>,
    internal_derivation_map: HashMap<String, String>,
}

pub struct Wallet {
    seed: [u8; 64],
    bip32: AddressPurpose,
    bip44: AddressPurpose,
    bip49: AddressPurpose,
    bip84: AddressPurpose,
    bip86: AddressPurpose,
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
    pub fn new(seed: [u8; 64]) -> Self {
        Wallet {
            seed,
            bip32: AddressPurpose::default(),
            bip44: AddressPurpose::default(),
            bip49: AddressPurpose::default(),
            bip84: AddressPurpose::default(),
            bip86: AddressPurpose::default(),
        }
    }
    pub fn get_balance(&self) {
        self.bip32.get_balance();
        self.bip44.get_balance();
        self.bip49.get_balance();
        self.bip84.get_balance();
        self.bip86.get_balance();
    }
}

impl AddressPurpose {
    pub fn get_balance(&self) {
        for address in self.external_derivation_map.keys() {

        }
        for address in self.internal_derivation_map.keys() {

        }

    }
    pub fn get_new_address(&self, situation: AddressLocation, seed: &[u8; 64], purpose: &str) {
        match purpose {
            bip32path => {},
            bip44path => {},

        }
        
        
        match situation {
            AddressLocation::EXTERNAL => {
                let path = bip44path.to_string() + &self.internal_index.to_string();
            },
            AddressLocation::INTERNAL => {}
        }
    }
}

pub fn derive_addresses() -> Result<()> {
    // let rootXprv = XPrv::new(&seed);
    let seed = generate_seed()?;
    
    let index = "0";
    // let xprv44 = XPrv::derive_from_path(&seed, &(bip44path + index).parse()?)?;
    // let xprv49 = XPrv::derive_from_path(&seed, &(bip49path + index).parse()?)?;
    // let xprv84 = XPrv::derive_from_path(&seed, &(bip84path + index).parse()?)?;
    // let xprv86 = XPrv::derive_from_path(&seed, &(bip86path + index).parse()?)?;
    // xprv44.zeroize();
    // println!("xprv44: {:?}", xprv44);
    // println!("xprv49: {:?}", xprv49);
    // println!("xprv84: {:?}", xprv84);
    // println!("xprv86: {:?}", xprv86);
    Ok(())
}
