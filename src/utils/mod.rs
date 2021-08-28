mod auth_info;
pub mod credentials;
pub mod channels;
pub mod message_cache;

pub use auth_info::AuthInfo;
use iota_identity_lib::iota::Credential;
use std::sync::MutexGuard;
use iota_identity_lib::api::{IdentityManager, Validator};
use bioenpro4to_channel_manager::utils::{current_time_secs, timestamp_to_date};
use chrono::Datelike;
use crate::utils::credentials::CredentialProperties;
use crate::errors::ResponseError;
use bioenpro4to_channel_manager::channels::Category;

pub fn extract_properties(cred: &Credential) -> Result<CredentialProperties, ResponseError>{
    match CredentialProperties::from_credential(&cred){
        None => Err(ResponseError::BadRequest("Bad credential properties format".into())),
        Some(prop) => Ok(prop),
    }
}

pub async fn validate_credential<'a>(cred: &Credential, manager: &MutexGuard<'a, IdentityManager>) -> Result<(), ResponseError>{
    // check if the credential is still valid
    match cred.expiration_date{
        None => return Err(ResponseError::BadRequest("Bad credential format".into())),
        Some(timestamp) => {
            if timestamp.to_unix() <= current_time_secs(){
                return Err(ResponseError::Unauthorized("Expired credential".into()))
            }
        }
    };

    // validate the credential
    let expected_did = manager.get_identity("santer reply").unwrap();
    let is_valid = match Validator::validate_credential(cred, expected_did.id()).await{
        Ok(res) => res,
        Err(_) => return Err(ResponseError::Internal("Error while validating credential".into()))
    };
    if !is_valid {
        return Err(ResponseError::Unauthorized("Invalid Credential".into()));
    }
    Ok(())
}

pub fn extract_date(timestamp: i64) -> (u16, u16, u16){
    let date = timestamp_to_date(timestamp, false);
    (date.day() as u16, date.month() as u16, date.year() as u16)
}

pub fn match_category(category: &str) -> Result<Category, ResponseError>{
    match Category::from_string(category){
        None => Err(ResponseError::BadRequest("Unknown category".into())),
        Some(c) => Ok(c)
    }
}
