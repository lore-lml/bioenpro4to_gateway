use crate::utils::channel_authorization::ChannelAuthorization;
use crate::utils::{extract_properties, validate_credential, extract_date, match_category};
use crate::errors::ResponseError;
use actix_web::web;
use crate::environment::AppState;
use deadpool_postgres::Pool;
use crate::database::db::DBManager;

pub async fn create_daily_channel(request: ChannelAuthorization, state: web::Data<AppState>, pool: web::Data<Pool>) -> Result<(), ResponseError>{
    // TODO: update channel cache when new channels are created
    let db_manager = DBManager::new(pool);
    let cred = request.cred();
    let prop = extract_properties(cred)?;

    // check if the credential is still valid
    let manager = state.identity.lock().unwrap();
    let mut root = state.root.lock().unwrap();
    let expected_did = manager.get_identity(state.config.identity_issuer_name()).unwrap();
    validate_credential(cred, expected_did, &db_manager, state.config.is_main_net()).await?;

    // creating the daily channel with the specified date
    let (day, month, year) = extract_date(request.day_timestamp());

    let category = match_category(prop.category())?;

    root.new_daily_actor_channel(
        category, prop.actor_id(), &request.channel_psw(),
        day, month, year).await
        .map_or_else(|e| Err(ResponseError::Internal(e.to_string())), |_| Ok(()))
}

pub async fn get_daily_channel(request: ChannelAuthorization, state: web::Data<AppState>, pool: web::Data<Pool>) -> Result<String, ResponseError>{
    let db_manager = DBManager::new(pool);
    let cred = request.cred();
    let prop = extract_properties(cred)?;

    // check if the credential is still valid
    let manager = state.identity.lock().unwrap();
    let mut root = state.root.lock().unwrap();
    let expected_did = manager.get_identity(state.config.identity_issuer_name()).unwrap();
    validate_credential(cred, expected_did, &db_manager, state.config.is_main_net()).await?;

    // creating the daily channel with the specified date
    let (day, month, year) = extract_date(request.day_timestamp());

    let category = match_category(prop.category())?;

    root.serialize_daily_actor_channel(
        category,
        prop.actor_id(),
        request.channel_psw(),
        day, month, year
    ).await
        .map_err(|x| ResponseError::BadRequest(x.to_string()))
}
