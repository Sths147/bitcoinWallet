use rustywallet_electrum::ElectrumClient;
use anyhow::Result;

pub(crate) struct ElectrumBackend {
    pub(crate)client: ElectrumClient
}

impl ElectrumBackend {
    pub async fn new(location: &str) -> Result<Self> {
        let res = ElectrumBackend { client: ElectrumClient::new(location).await? };
        Ok(res)
    }
}