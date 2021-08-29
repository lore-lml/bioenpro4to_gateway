use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActorUpdate{
    actor_id: String,
    category: String,
    timestamp: i64
}

#[allow(dead_code)]
impl ActorUpdate{
    pub fn new(actor_id: &str, category: &str, timestamp: i64) -> Self {
        let actor_id = actor_id.to_string();
        let category = category.to_string();
        ActorUpdate { actor_id, category, timestamp }
    }

    pub fn actor_id(&self) -> &str {
        &self.actor_id
    }
    pub fn category(&self) -> &str {
        &self.category
    }
    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }
}
