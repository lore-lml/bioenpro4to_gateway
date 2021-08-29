use actix_web::web;
use crate::utils::match_category;
use crate::environment::AppState;
use bioenpro4to_channel_manager::channels::{ActorChannelInfo, DailyChannelInfo, ChannelInfo};
use crate::errors::ResponseError;
use std::collections::HashMap;
use serde_json::Value;
use bioenpro4to_channel_manager::utils::{timestamp_to_date_string, current_time_secs};

pub async fn actors_last_updates(count: u16, state: web::Data<AppState>) -> Result<Vec<HashMap<String, Value>>, ResponseError>{
    let current_date = timestamp_to_date_string(current_time_secs(), false);
    let actors: Vec<ActorChannelInfo> = ["trucks", "weighing_scales", "biocells"].iter()
        .flat_map(|category| {
            actors_of_category(category, state.clone()).unwrap_or(vec![])
        }).collect();

    let daily_channels = actors.iter()
        .flat_map(|a| {
            channels_of_actor(a.category(), a.actor_id(), state.clone()).unwrap_or(vec![])
                .into_iter().filter(|ch| ch.creation_date() == current_date)
                .map(|ch| ch.address().clone())
                .collect::<Vec<ChannelInfo>>()
        })
        .collect::<Vec<ChannelInfo>>();

    let mut found = vec![];
    let mut cache = state.msg_cache.lock().unwrap();
    for ch in daily_channels {
        found.append( &mut cache.get(&ch.to_string()).await?);
    }

    let mut found = found.into_iter()
        .filter(|m| {
            match m.get("timestamp"){
                None => false,
                Some(value) => value.as_i64().is_some()
            }
        })
        .collect::<Vec<HashMap<String, Value>>>();
    found.sort_by_key(|m| -m.get("timestamp").unwrap().as_i64().unwrap());
    let found = found.into_iter()
        .take(count as usize)
        .collect::<Vec<HashMap<String, Value>>>();


    Ok(found)
}

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
    Ok(cache.get(&selected_info.to_string()).await?)
}
