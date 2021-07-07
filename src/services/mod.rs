pub mod credentials;

use std::sync::Mutex;
use bioenpro4to_channel_manager::channels::root_channel::RootChannel;
use iota_identity_lib::api::IdentityManager;
use actix_web::HttpRequest;

pub struct AppState{
    pub root: Mutex<RootChannel>,
    pub identity: Mutex<IdentityManager>
}

impl AppState{
    pub fn new(root: RootChannel, identity: IdentityManager) -> Self {
        AppState { root: Mutex::new(root), identity: Mutex::new(identity) }
    }
}

pub struct AuthInfo{
    id: String,
    did: String,
}

impl AuthInfo{
    pub fn from_http_request(req: &HttpRequest) -> Option<Self>{
        let id = req.headers().get("id")?.to_str().ok()?.to_owned();
        let did = req.headers().get("did")?.to_str().ok()?.to_owned();
        Some(AuthInfo{id, did})
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn did(&self) -> &str {
        &self.did
    }
}
