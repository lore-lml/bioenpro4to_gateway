use std::collections::HashMap;
use bioenpro4to_channel_manager::channels::{MessageReader, ChannelInfo};
use serde_json::Value;
use crate::errors::ResponseError;

pub struct MessageCache{
    cache: HashMap<String, MessageReader>,
    mainnet: bool,
    msgs_update_time: i64,
}

impl MessageCache{
    pub fn new(mainnet: bool, msgs_update_time: i64) -> Self {
        MessageCache { cache: HashMap::default(), mainnet, msgs_update_time }
    }

    pub async fn get(&mut self, key: &str) -> Result<Vec<HashMap<String, Value>>, ResponseError>{
        //Tries to get
        let mr = match self.cache.get_mut(key){
            None => {
                let split: Vec<&str> = key.split(":").collect();
                if split.len() != 2 {
                    return Err(ResponseError::BadRequest("wrong key format".to_string()));
                }
                let info = ChannelInfo::new(split[0].to_string(), split[1].to_string());
                MessageReader::new(&info, self.mainnet).await
                    .map_err(|err| ResponseError::NotFound(err.to_string()))?
            }
            Some(reader) => {
                if reader.last_updates_seconds_ago() >= self.msgs_update_time{
                    reader.read_messages().await
                        .map_err(|err| ResponseError::Internal(err.to_string()))?;
                }
                return Ok(reader.msgs().clone())
            }
        };
        self.cache.insert(key.to_string(), mr);
        Ok(self.cache.get(key).unwrap().msgs().clone())
    }
}
