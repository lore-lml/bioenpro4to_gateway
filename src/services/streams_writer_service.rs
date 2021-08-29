use crate::utils::channels::ChannelCreationRequest;
use crate::utils::{extract_properties, validate_credential, extract_date, match_category};
use crate::errors::ResponseError;
use actix_web::web;
use crate::environment::AppState;

pub async fn create_daily_channel(request: ChannelCreationRequest, state: web::Data<AppState>) -> Result<(), ResponseError>{
    let cred = request.cred();
    let prop = extract_properties(cred)?;

    // check if the credential is still valid
    let manager = state.identity.lock().unwrap();
    let mut root = state.root.lock().unwrap();
    validate_credential(cred, &manager).await?;

    // creating the daily channel with the specified date
    let (day, month, year) = extract_date(request.day_timestamp());

    let category = match_category(prop.category())?;

    match root.new_daily_actor_channel(
        category, prop.actor_id(), &request.psw(),
        day, month, year).await{
        Ok(_) => Ok(()),
        Err(e) => Err(ResponseError::Internal(e.to_string()))
    }
}

pub async fn get_daily_channel(request: ChannelCreationRequest, state: web::Data<AppState>) -> Result<String, ResponseError>{
    // extract properties from credential
    let cred = request.cred();
    let prop = extract_properties(cred)?;

    // check if the credential is still valid
    let manager = state.identity.lock().unwrap();
    let mut root = state.root.lock().unwrap();
    validate_credential(cred, &manager).await?;

    // creating the daily channel with the specified date
    let (day, month, year) = extract_date(request.day_timestamp());

    let category = match_category(prop.category())?;

    root.serialize_daily_actor_channel(
        category,
        prop.actor_id(),
        request.psw(),
        day, month, year
    ).await
        .map_err(|x| ResponseError::BadRequest(x.to_string()))
}