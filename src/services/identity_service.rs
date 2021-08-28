use crate::utils::AuthInfo;
use actix_web::web;
use crate::environment::AppState;
use iota_identity_lib::iota::{IotaDID, json, Credential};
use crate::errors::ResponseError;
use iota_identity_lib::api::Validator;
use serde_json::Value;
use deadpool_postgres::Pool;
use crate::database::db::DBManager;
use bioenpro4to_channel_manager::utils::hash_string;

pub async fn get_credential(auth: AuthInfo, state: web::Data<AppState>, pool: web::Data<Pool>) -> Result<Credential, ResponseError>{
    let db_manager = DBManager::new(pool);
    let actor = db_manager.get_actor(auth.id()).await.map_err(|_| ResponseError::BadRequest("Wrong user_id or password".to_string()))?;
    if actor.psw().to_string() != hash_string(auth.psw()){
        return Err(ResponseError::BadRequest("Wrong user_id or password".to_string()));
    }
    let actor_did = IotaDID::parse(auth.did()).map_err(|_| ResponseError::BadRequest("Wrong Did format".into()))?;
    if actor.did().to_string() != actor_did.to_string(){
        return Err(ResponseError::BadRequest(format!("The provided did doesn't correspond to the actor {}", actor.id())))
    }

    let document_valid =Validator::is_document_valid(actor.did(), state.config.is_main_net()).await
        .map_err(|_| ResponseError::Internal("Something went wrong during did document validation".to_string()))?;
    if !document_valid{
        return Err(ResponseError::BadRequest("The provided did is not on the tangle".to_string()));
    }

    let category = actor.category();
    let manager = state.identity.lock().unwrap();

    let cred = manager.issue_credential_as(state.config.identity_issuer_name(), &actor_did, "ChannelWriteAuth", json!({
        "channel_authorization":{
            "actor_id": auth.id(),
            "category": &category.to_string(),
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
