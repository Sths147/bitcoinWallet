use corepc_client::client_sync::{Auth, v28::Client};

trait BitcoinBackend {
    fn list_unspent(&self);
    fn get_balance(&self);
    fn recover(&self);
    fn broadcast_transaction(&self);
}

pub struct BitcoinCoreRpc {
    pub client: Client,
}

impl BitcoinCoreRpc {
    pub fn new(url: &str, rpc_username: String, rpc_password: String) -> Self {
        BitcoinCoreRpc {
            client: Client::new_with_auth(url, Auth::UserPass(rpc_username, rpc_password))
                .expect("Error creating Bitcoin core client"),
        }
    }
}

impl BitcoinBackend for BitcoinCoreRpc {
    fn list_unspent(&self) {
        
    }
    fn broadcast_transaction(&self) {
        
    }
    fn get_balance(&self) {
        
    }
    fn recover(&self) {

        //etape 1: générer 20 addresses de external_chain
        //etape 2: les envoyer à bitcoin_core pour qu'il les watch
        //etape 3: vérifier si les addresses sont utilisées
        //si 20 dernières addresses sont utilisées, continuer, sinon break;
        //une fois fini faire de même avec les 20 addresses de internal_chain
    }
}