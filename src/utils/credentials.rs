use iota_identity_lib::iota::Credential;

pub struct CredentialProperties{
    actor_id: String,
    category: String,
}

impl CredentialProperties{
    pub fn from_credential(cred: &Credential) -> Option<Self>{
        let prop = &cred.credential_subject.get(0)?.properties
            .get("channel_authorization")?.as_object()?;
        let category = prop.get("category")?.to_string().replace("\"", "");
        let actor_id = prop.get("actor_id")?.to_string().replace("\"", "");
        Some(CredentialProperties{actor_id, category})
    }

    pub fn actor_id(&self) -> &str {
        &self.actor_id
    }
    pub fn category(&self) -> &str {
        &self.category
    }
}
