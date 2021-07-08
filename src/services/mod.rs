pub mod credentials;
pub mod channels;

use actix_web::HttpRequest;

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
