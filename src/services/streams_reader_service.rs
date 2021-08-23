use actix_web::web;
use crate::utils::match_category;
use crate::environment::AppState;
use bioenpro4to_channel_manager::channels::{ActorChannelInfo, DailyChannelInfo};
use crate::errors::ResponseError;
use std::collections::HashMap;
use serde_json::Value;

pub fn actors_of_category(category: &str, state: web::Data<AppState>) -> Result<Vec<ActorChannelInfo>, ResponseError>{
    let category = match_category(category)?;
    let root = state.root.lock().unwrap();
    Ok(root.actors_of_category(category.clone()))
}

pub fn channels_of_actor(category: &str, actor_id: &str, state: web::Data<AppState>) -> Result<Vec<DailyChannelInfo>, ResponseError>{
    let category = match_category(category)?;
    let root = state.root.lock().unwrap();
    Ok(root.channels_of_actor(category, actor_id))
}

pub async fn messages_of_channel_of_actor(category: &str, actor_id: &str, date: &str, state: web::Data<AppState>) -> Result<Vec<HashMap<String, Value>>, ResponseError>{
    let daily_ch_info = channels_of_actor(category, actor_id, state.clone())?;
    let selected_info = daily_ch_info.iter()
        .find(|ch| ch.creation_date() == date.to_string());
    let selected_info = match selected_info{
        None => return Err(ResponseError::BadRequest(format!("There is no channel in date {} for actor {}", date, actor_id))),
        Some(ch) => ch.address()
    };
    let mut cache = state.msg_cache.lock().unwrap();
    Ok(cache.get(
        &format!("{}:{}", selected_info.channel_id(), selected_info.announce_id())
    ).await?)
}
