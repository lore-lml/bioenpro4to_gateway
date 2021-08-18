use actix_web::web;
use crate::utils::match_category;
use crate::environment::AppState;
use bioenpro4to_channel_manager::channels::{ActorChannelInfo, DailyChannelInfo};
use crate::errors::ResponseError;

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

pub async fn messages_of_channel_of_actor(category: &str, actor_id: &str, date: &str, state: web::Data<AppState>) -> Result<Vec<String>, ResponseError>{
    let category = match_category(category)?;
    let root = state.root.lock().unwrap();
    root.daily_channel_info(category, actor_id, date).await
        .map_err(|err| ResponseError::BadRequest(err.to_string()))
}
