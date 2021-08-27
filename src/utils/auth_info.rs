use actix_web::HttpRequest;
use bioenpro4to_channel_manager::channels::Category;
use crate::utils::match_category;

pub struct AuthInfo{
    id: String,
    did: String,
    nonce: Option<String>,
    category: Category
}

impl AuthInfo{
    pub fn from_http_request(req: &HttpRequest) -> Option<Self>{
        let id = req.headers().get("id")?.to_str().ok()?.to_owned();
        let did = req.headers().get("did")?.to_str().ok()?.to_owned();
        let nonce = req.headers().get("nonce")
            .map_or(None, |header| header.to_str().ok())
            .map_or(None, |nonce| Some(nonce.to_string()));
        let category = req.headers().get("category")?.to_str().ok()?.to_owned();
        let category = match_category(&category).ok()?;
        Some(AuthInfo{id, did, nonce, category})
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn did(&self) -> &str {
        &self.did
    }
    pub fn nonce(&self) -> &Option<String> {
        &self.nonce
    }
    pub fn category(&self) -> &Category {
        &self.category
    }
}
