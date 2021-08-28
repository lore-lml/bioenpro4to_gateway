use actix_web::HttpRequest;

pub struct AuthInfo{
    id: String,
    psw: String,
    did: String,
}

impl AuthInfo{
    pub fn from_http_request(req: &HttpRequest) -> Option<Self>{
        let id = req.headers().get("id")?.to_str().ok()?.to_owned();
        let did = req.headers().get("did")?.to_str().ok()?.to_owned();
        let psw = req.headers().get("psw")?.to_str().ok()?.to_owned();
        Some(AuthInfo{id, psw, did})
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn psw(&self) -> &str {
        &self.psw
    }
    pub fn did(&self) -> &str {
        &self.did
    }
}
