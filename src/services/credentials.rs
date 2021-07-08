use actix_web::{web, get, Responder, HttpResponse, HttpRequest};
use crate::services::{AuthInfo, AppState};
use iota_identity_lib::iota::{IotaDID, json, Credential};
use bioenpro4to_channel_manager::channels::Category;
use std::ops::Deref;
use iota_identity_lib::api::Validator;


#[get("/channel-credential")]
pub async fn get_credential(req: HttpRequest, data: web::Data<AppState>) -> impl Responder{
    let auth = match AuthInfo::from_http_request(&req){
        None => return HttpResponse::BadRequest().body("Wrong Auth header format"),
        Some(auth) => auth
    };

    /*
    CHECKING ON A DATABASE IF THE ID AND THE DID ARE RELATED
     */
    let category = Category::Trucks.to_string();
    let manager = data.identity.lock().unwrap();
    let subject_did = match IotaDID::parse(auth.did()){
        Ok(did) => did,
        Err(_) => return HttpResponse::BadRequest().body("Wrong Did format"),
    };
    let cred = manager.issue_credential_as("santer reply", &subject_did, "ChannelWriteAuth", json!({
        "channel_authorization":{
            "actor_id": auth.id(),
            "category": &category,
        }
    })).await;
    let cred = match cred{
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong while building credentials")
    };

    HttpResponse::Ok().json(cred)
}

#[get("/is-credential-valid")]
pub async fn is_credential_valid(data: web::Data<AppState>, cred: web::Json<Credential>) -> impl Responder{
    let cred = cred.deref();
    let manager = data.identity.lock().unwrap();
    let expected_did = manager.get_identity("santer reply").unwrap();
    let validation = match Validator::validate_credential(cred, expected_did.id()).await{
        Ok(res) => res,
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong while validating")
    };
    let res = json!({
        "is_valid": validation
    });
    HttpResponse::Ok().json(res)
}
