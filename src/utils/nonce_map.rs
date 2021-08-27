use std::collections::HashMap;
use bioenpro4to_channel_manager::channels::Category;
use rand::Rng;

pub struct NonceMap{
    map: HashMap<(String, String, Category), String>,
}

impl NonceMap{
    fn generate_nonce() -> String{
        const UPPER_CASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        const LOWER_CASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        const NUMBERS: &[u8] = b"1234567890";
        const SYMBOLS: &[u8] = b"!@#$%^&*()_+><?;:[]";
        const NONCE_LEN: usize = 8;

        let charset: Vec<u8> = [UPPER_CASE, LOWER_CASE, NUMBERS, SYMBOLS].concat();
        let mut rng = rand::thread_rng();

        let nonce: String = (0..NONCE_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx] as char
            })
            .collect();

        nonce
    }
    pub fn new() -> Self {
        NonceMap { map: HashMap::default() }
    }

    pub fn insert_nonce_for_actor(&mut self, actor_id: &str, actor_did: &str, category: &Category) -> String{
        let key = (actor_id.to_lowercase(), actor_did.to_lowercase(), category.clone());
        let nonce = NonceMap::generate_nonce();
        self.map.insert(key, nonce.clone());
        return nonce
    }

    pub fn pop_nonce(&mut self, actor_id: &str, actor_did: &str, category: &Category) -> Option<String>{
        let key = (actor_id.to_lowercase(), actor_did.to_lowercase(), category.clone());
        self.map.remove(&key)
    }
}
