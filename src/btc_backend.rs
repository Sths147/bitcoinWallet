use corepc_client::client_sync::v28::Client;

trait BitcoinBackend {
    fn list_unspent(&self);
    fn get_balance(&self);
    fn recover(&self);
    fn broadcast_transaction(&self);
}

struct BitcoinCoreRpc {
    client: Client,
}