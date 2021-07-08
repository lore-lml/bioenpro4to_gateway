use actix_web::{web, post, Responder, HttpResponse};
use iota_identity_lib::api::Validator;
use iota_identity_lib::iota::Credential;
use serde::{Serialize, Deserialize};
use bioenpro4to_channel_manager::utils::{timestamp_to_date, current_time_secs};
use bioenpro4to_channel_manager::channels::Category;
use chrono::Datelike;

use crate::environment::AppState;

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
    let prop = match CredentialProperties::from_credential(&cred){
        None => return HttpResponse::BadRequest().body("Bad credential properties format"),
        Some(prop) => prop,
    };

    // check if the credential is still valid
    match body.cred.expiration_date{
        None => return HttpResponse::BadRequest().body("Bad credential format"),
        Some(timestamp) => {
            if timestamp.to_unix() <= current_time_secs(){
                return HttpResponse::Unauthorized().body("Expired credential")
            }
        }
    };

    // validate the credential
    let manager = data.identity.lock().unwrap();
    let expected_did = manager.get_identity("santer reply").unwrap();
    let is_valid = match Validator::validate_credential(cred, expected_did.id()).await{
        Ok(res) => res,
        Err(_) => return HttpResponse::InternalServerError().body("Error while validating credential")
    };
    if !is_valid {
        return HttpResponse::Unauthorized().body("Invalid Credential")
    }

    // creating the daily channel with the specified date
    let mut root = data.root.lock().unwrap();
    let date = timestamp_to_date(body.day_timestamp, false);
    let (day, month, year) = (date.day() as u16, date.month() as u16, date.year() as u16);

    let category = match Category::from_string(&prop.category){
        None => return HttpResponse::BadRequest().body("Unknown category"),
        Some(c) => c
    };

    let daily_channel = match root.get_or_create_daily_actor_channel(
        category, &prop.actor_id, &body.psw,
        day, month, year).await{
        Ok(ch) => ch,
        Err(_) => return HttpResponse::InternalServerError().body("Error during channel creation")
    };

    HttpResponse::Ok().json(daily_channel.channel_info())
}
