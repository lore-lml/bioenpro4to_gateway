use iota_identity_lib::iota::Credential;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ChannelCreationRequest{
    cred: Credential,
    psw: String,
    day_timestamp: i64,
}

impl ChannelCreationRequest{
    pub fn cred(&self) -> &Credential {
        &self.cred
    }
    pub fn psw(&self) -> &str {
        &self.psw
    }
    pub fn day_timestamp(&self) -> i64 {
        self.day_timestamp
    }
}
