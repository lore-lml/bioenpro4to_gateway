use crate::utils::AuthInfo;
use actix_web::web;
use crate::environment::AppState;
use iota_identity_lib::iota::{IotaDID, json, Credential};
use crate::errors::ResponseError;
use iota_identity_lib::api::Validator;
use serde_json::Value;
use deadpool_postgres::Pool;
use crate::database::db::DBManager;
use bioenpro4to_channel_manager::channels::Category;

pub async fn get_nonce_for_actor(auth: AuthInfo, state: web::Data<AppState>, pool: web::Data<Pool>) -> Result<String, ResponseError>{
    if auth.nonce().is_some(){
        return Err(ResponseError::BadRequest("Nonce is not empty".to_string()));
    }
    let db_manager = DBManager::new(pool);
    let (actor_exists, did) = match auth.category(){
        Category::Trucks => db_manager.get_truck(auth.id()).await
                .map_or((false, None), |truck| (true, Some(truck.did().to_string()))),

        Category::Scales => db_manager.get_scale(auth.id()).await
            .map_or((false, None), |scale| (true, Some(scale.did().to_string()))),

        Category::BioCells => db_manager.get_biocell(auth.id()).await
            .map_or((false, None), |cell| (true, Some(cell.did().to_string()))),
    };

    if !actor_exists{
        return Err(ResponseError::BadRequest(format!("Actor {} not found in category <{}>", auth.id(), auth.category().to_string())));
    }
    if did.unwrap().to_lowercase() != auth.did().to_lowercase(){
        return Err(ResponseError::BadRequest(format!("The provided did for actor {} in category <{}> does not match", auth.id(), auth.category().to_string())));
    }

    let mut nonce_map = state.nonce_map.lock().unwrap();
    Ok(nonce_map.insert_nonce_for_actor(auth.id(), auth.did(), auth.category()))
}

pub async fn get_credential(auth: AuthInfo, state: web::Data<AppState>, pool: web::Data<Pool>) -> Result<Credential, ResponseError>{
    let db_manager = DBManager::new(pool);
    let user = db_manager.get_user(auth.id()).await?;
    /*if user.psw().to_string() != hash_string(auth.psw()){
        return Err(ResponseError::BadRequest("Wrong user_id or password".to_string()));
    }*/
    let user_did = match user.did(){
        None => return Err(ResponseError::BadRequest(format!("User {} has no valid digital identity", user.id()))),
        Some(did) => did.to_lowercase()
    };
    if user_did != auth.did().to_lowercase(){
        return Err(ResponseError::BadRequest(format!("The provided did doesn't correspond to the user {}", user.id())))
    }

    let category = auth.category();
    let manager = state.identity.lock().unwrap();
    let subject_did = match IotaDID::parse(auth.did()){
        Ok(did) => did,
        Err(_) => return Err(ResponseError::BadRequest("Wrong Did format".into())),
    };
    let cred = manager.issue_credential_as(state.config.identity_issuer_name(), &subject_did, "ChannelWriteAuth", json!({
        "channel_authorization":{
            "actor_id": auth.id(),
            "category": category,
        }
    })).await;

    match cred{
        Ok(c) => Ok(c),
        Err(e) => return Err(ResponseError::Internal(e.to_string()))
    }
}

pub async fn is_credential_valid(cred: Credential, state: web::Data<AppState>) -> Result<Value, ResponseError>{
    let manager = state.identity.lock().unwrap();
    let expected_did = manager.get_identity("santer reply").unwrap();
    let validation = match Validator::validate_credential(&cred, expected_did.id()).await{
        Ok(res) => res,
        Err(_) => return Err(ResponseError::Internal("Something went wrong while validating".into())),
    };
    Ok(
        json!({
            "is_valid": validation
        })
    )
}
