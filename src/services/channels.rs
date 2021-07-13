use actix_web::{web, post, get, Responder, HttpResponse};
use iota_identity_lib::api::{Validator, IdentityManager};
use iota_identity_lib::iota::Credential;
use serde::{Serialize, Deserialize};
use bioenpro4to_channel_manager::utils::{timestamp_to_date, current_time_secs};
use bioenpro4to_channel_manager::channels::Category;
use chrono::Datelike;

use crate::environment::AppState;
use std::sync::MutexGuard;

#[derive(Serialize, Deserialize)]
pub struct ChannelCreationRequest{
    cred: Credential,
    psw: String,
    day_timestamp: i64,
}

struct CredentialProperties{
    actor_id: String,
    category: String,
}

impl CredentialProperties{
    fn from_credential(cred: &Credential) -> Option<Self>{
        let prop = &cred.credential_subject.get(0)?.properties
            .get("channel_authorization")?.as_object()?;
        let category = prop.get("category")?.to_string().replace("\"", "");
        let actor_id = prop.get("actor_id")?.to_string().replace("\"", "");
        Some(CredentialProperties{actor_id, category})
    }
}

#[post("/daily-channel")]
pub async fn create_daily_channel(body: web::Json<ChannelCreationRequest>, data: web::Data<AppState>) -> impl Responder{
    // extract properties from credential
    let cred = &body.cred;
    let prop = match extract_properties(cred){
        Ok(prop) => prop,
        Err(resp) => return resp
    };

    // check if the credential is still valid
    let manager = data.identity.lock().unwrap();
    let mut root = data.root.lock().unwrap();
    match validate_credential(cred, &manager).await{
        Ok(_) => {},
        Err(resp) => return resp
    };

    // creating the daily channel with the specified date
    let (day, month, year) = extract_date(body.day_timestamp);

    let category = match Category::from_string(&prop.category){
        None => return HttpResponse::BadRequest().body("Unknown category"),
        Some(c) => c
    };

    let daily_channel = match root.new_daily_actor_channel(
        category, &prop.actor_id, &body.psw,
        day, month, year).await{
        Ok(ch) => ch,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string())
    };

    HttpResponse::Ok().json(daily_channel.channel_info())
}

#[get("/daily-channel")]
pub async fn get_daily_channel(body: web::Json<ChannelCreationRequest>, data: web::Data<AppState>) -> impl Responder{
    // extract properties from credential
    let cred = &body.cred;
    let prop = match extract_properties(cred){
        Ok(prop) => prop,
        Err(resp) => return resp
    };

    // check if the credential is still valid
    let manager = data.identity.lock().unwrap();
    let mut root = data.root.lock().unwrap();
    match validate_credential(cred, &manager).await{
        Ok(_) => {},
        Err(resp) => return resp
    };

    // creating the daily channel with the specified date
    let (day, month, year) = extract_date(body.day_timestamp);

    let category = match Category::from_string(&prop.category){
        None => return HttpResponse::BadRequest().body("Unknown category"),
        Some(c) => c
    };

    let daily_channel = match root.get_daily_actor_channel(
        category, &prop.actor_id, &body.psw,
        day, month, year).await{
        Ok(ch) => ch,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string())
    };

    HttpResponse::Ok().json(daily_channel.channel_info())
}

fn extract_properties(cred: &Credential) -> Result<CredentialProperties, HttpResponse>{
    match CredentialProperties::from_credential(&cred){
        None => Err(HttpResponse::BadRequest().body("Bad credential properties format")),
        Some(prop) => Ok(prop),
    }
}

async fn validate_credential<'a>(cred: &Credential, manager: &MutexGuard<'a, IdentityManager>) -> Result<(), HttpResponse>{
    // check if the credential is still valid
    match cred.expiration_date{
        None => return Err(HttpResponse::BadRequest().body("Bad credential format")),
        Some(timestamp) => {
            if timestamp.to_unix() <= current_time_secs(){
                return Err(HttpResponse::Unauthorized().body("Expired credential"))
            }
        }
    };

    // validate the credential
    let expected_did = manager.get_identity("santer reply").unwrap();
    let is_valid = match Validator::validate_credential(cred, expected_did.id()).await{
        Ok(res) => res,
        Err(_) => return Err(HttpResponse::InternalServerError().body("Error while validating credential"))
    };
    if !is_valid {
        return Err(HttpResponse::Unauthorized().body("Invalid Credential"));
    }
    Ok(())
}

fn extract_date(timestamp: i64) -> (u16, u16, u16){
    let date = timestamp_to_date(timestamp, false);
    (date.day() as u16, date.month() as u16, date.year() as u16)
}
