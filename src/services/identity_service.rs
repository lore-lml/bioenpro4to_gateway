use crate::utils::AuthInfo;
use actix_web::web;
use crate::environment::AppState;
use iota_identity_lib::iota::{IotaDID, json, Credential};
use crate::errors::ResponseError;
use bioenpro4to_channel_manager::channels::Category;
use iota_identity_lib::api::Validator;
use serde_json::Value;

pub async fn get_credential(auth: AuthInfo, state: web::Data<AppState>) -> Result<Credential, ResponseError>{
    /*
    CHECKING ON A DATABASE IF THE ID AND THE DID ARE RELATED
     */

    let category = Category::Trucks.to_string();
    let manager = state.identity.lock().unwrap();
    let subject_did = match IotaDID::parse(auth.did()){
        Ok(did) => did,
        Err(_) => return Err(ResponseError::BadRequest("Wrong Did format".into())),
    };
    let cred = manager.issue_credential_as(state.config.identity_issuer_name(), &subject_did, "ChannelWriteAuth", json!({
        "channel_authorization":{
            "actor_id": auth.id(),
            "category": &category,
        }
    })).await;
    match cred{
        Ok(c) => Ok(c),
        Err(_) => return Err(ResponseError::Internal("Something went wrong while building credentials".into()))
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
