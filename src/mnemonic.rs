use anyhow::{Result, anyhow};
use bip32::{
    ChildNumber, DerivationPath, ExtendedPublicKey, PublicKey, XPrv, XPub,
    secp256k1::ecdsa::VerifyingKey,
};
use bip39::Mnemonic;
// use corepc_client::bitcoin::hashes::hash160::Hash;
use bitcoin::{
    Address, CompressedPublicKey, Network,
    hashes::{Hash, hash160},
};
use rand_core::{OsRng, RngCore};
use std::collections::HashMap;
use zeroize::{Zeroize, Zeroizing};
use rustywallet_electrum::Balance;
use crate::BitcoinCoreRpc;
use crate::electrum_backend::ElectrumBackend;
// use bitcoin_hashes::hash160;

const GAP_LIMIT: u16 = 20;
const BIP_84_PATH: &str = "m/44'/0'/0'";
const NETWORK: Network = Network::Testnet4;
/*
For each address purpose, 2 pairs of index, derivation map are kept
derivation map to keep track of address/derivation paths matchings for spending
index to know the next derivation path to generate new address
*/

#[derive(Default)]
struct AddressPurpose {
    account: u32,
    internal_index: u32,
    external_index: u32,
}

pub struct Wallet {
    xprv_84: XPrv,
    xpub_84: XPub,
    account: u32,
    internal_index: u32,
    external_index: u32,
    address_map: HashMap<Address, String>,
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
        Err(e) => {
            key.zeroize();
            return Err(anyhow!(e));
        }
    };
    key.zeroize();
    // for (i, word) in mnemonic.words().enumerate() {
    //     println!("{}. {}", i + 1, word);
    // }
    let seed = mnemonic.to_seed_normalized("mnemonic");
    mnemonic.zeroize();
    Ok(seed)
}

pub fn get_account_xprv(mut seed: [u8; 64]) -> Result<XPrv> {
    let xprv_84 = XPrv::derive_from_path(&seed, &"m/84'/0'/0'".parse()?)?;
    seed.zeroize();
    Ok(xprv_84)
}

impl Wallet {
    pub fn new(xprv_84: XPrv) -> Self {
        let pubkey = xprv_84.public_key();
        Wallet {
            xprv_84,
            xpub_84: pubkey,
            account: 0,
            external_index: 0,
            internal_index: 0, // derivation_maps: HashMap::new(),
            address_map: HashMap::new(),
        }
    }
    pub fn new_p2wpkh(&mut self, keychain: u32) -> Result<Address> {
        self.check_keychain_index(keychain)?;
        let (child_xpub, index) = match keychain {
            0 => {
                self.external_index += 1;
                (self.xpub_84
                    .derive_child(ChildNumber::new(0, false)?)?
                    .derive_child(ChildNumber::new(self.external_index - 1, false)?)?
                , self.external_index - 1)
            }
            1 => {
                self.internal_index += 1;
                (self.xpub_84
                    .derive_child(ChildNumber::new(1, false)?)?
                    .derive_child(ChildNumber::new(self.internal_index - 1, false)?)?
                , self.internal_index - 1)
            }
            _ => return Err(anyhow!("Error, keychain must be 0 or 1 only")),
        };
        let pubkeyhash: hash160::Hash = hash160::Hash::hash(&child_xpub.public_key().to_bytes());
        println!("{:?}", pubkeyhash);
        let address = Address::p2wpkh(
            &CompressedPublicKey::from_slice(&child_xpub.public_key().to_bytes())?,
            NETWORK,
        );
        if self.address_map.contains_key(&address) {
            return Err(anyhow!("{} Address already in use", address))
        }
        let derivation_path = BIP_84_PATH.to_owned() + "/" + &keychain.to_string() + "/" + &index.to_string(); 
        self.address_map.insert(address.clone(), derivation_path);
        println!("Address: {:?}", address);
        Ok(address)
    }
    fn check_keychain_index(&self, keychain: u32) -> Result<()> {
        let (index, chaintype) = if keychain == 0 {
            (self.external_index, "external")
        } else {
            (self.internal_index, "internal")
        };
        if index == u32::MAX {
            return Err(anyhow!(
                "{} chain is at maximum index, cannot derive new address",
                chaintype
            ));
        };
        Ok(())
    }
    ///Generate 20 external chain addresses
    /// 
    pub async fn recover(&mut self, electrum: &ElectrumBackend) -> Result<()> {
        let mut addresses: Vec<String> = vec![];
        let mut gap = 20;
        let mut check_addresses: Vec<String> = vec![];
        loop {
            for _ in 0..gap {
                let address = self.new_p2wpkh(0)?.to_string();
                check_addresses.push(address);
            }
            let sendaddr: Vec<&str> = check_addresses.iter().map(|s| {s.as_str()}).collect::<Vec<&str>>();
            let res = electrum.client.get_balances(&sendaddr[..]).await?;
            println!("{:?}", res);
            gap = emptyaddresses(&res);
            println!("gap: {}", gap);
            if gap == 20 {
                break;
            }
            let (non_empty, empty) = check_addresses.split_at(20 - gap as usize);
            addresses.extend_from_slice(non_empty);
            check_addresses = empty.into();
        }
        Ok(())
        //etape 1: générer 20 addresses de external_chain
        //etape 2: les envoyer à bitcoin_core pour qu'il les watch
        //etape 3: vérifier si les addresses sont utilisées
        //si 20 dernières addresses sont utilisées, continuer, sinon break;
        //une fois fini faire de même avec les 20 addresses de internal_chain
    }
}

fn emptyaddresses(addresses: &[Balance]) -> u32 {
    addresses.iter()
        .rev()
        .take_while(|balance| balance.total() == 0)
        .count() as u32
}


/*
To do : Commencer par simple:
un seul account, uniquement BIP84
une seule structure
pour le recovery on fonctionne uniquement avec ce path la
*/

// impl AddressPurpose {
//     // get new address, je dois :
//     /*
//         Renvoyer le path
//      */
//     pub fn new(account: u32) -> Self {
//         AddressPurpose {
//             account,
//             ..Default::default()
//         }
//     }
//     fn get_new_address(&mut self, situation: u8) {
//         let index = if situation == 0 {&self.internal_index} else {&self.external_index};
//         let derivation_path =
//     }
//     fn increase_index(&mut self, situation: u8) {
//         if situation == 0 {
//             if self.external_index == u32::MAX {
//                 eprintln!("Max addresses reached for external chain with {}", self.path);
//             }
//             self.external_index += 1;
//         } else {
//             if self.internal_index == u32::MAX {
//                 eprintln!("Max addresses reached for external chain with {}", self.path);
//             }
//             self.internal_index += 1;
//         }
//     }
// }

// pub fn derive_addresses() -> Result<()> {
//     // let rootXprv = XPrv::new(&seed);
//     let seed = generate_seed()?;

//     let index = "0";
//     // let xprv44 = XPrv::derive_from_path(&seed, &(BIP44PATH + index).parse()?)?;
//     // let xprv49 = XPrv::derive_from_path(&seed, &(BIP49PATH + index).parse()?)?;
//     // let xprv84 = XPrv::derive_from_path(&seed, &(BIP84PATH + index).parse()?)?;
//     // let xprv86 = XPrv::derive_from_path(&seed, &(BIP86PATH + index).parse()?)?;
//     // xprv44.zeroize();
//     // println!("xprv44: {:?}", xprv44);
//     // println!("xprv49: {:?}", xprv49);
//     // println!("xprv84: {:?}", xprv84);
//     // println!("xprv86: {:?}", xprv86);
//     Ok(())
// }
