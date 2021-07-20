use actix_web::{web, get, HttpResponse, HttpRequest, ResponseError as AWResponseError};
use crate::services::identity_service;
use crate::environment::AppState;
use crate::utils::AuthInfo;
use crate::errors::ResponseError;
use iota_identity_lib::iota::Credential;


#[get("/channel-credential")]
pub async fn get_credential(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse{
    let auth = match AuthInfo::from_http_request(&req){
        None => return ResponseError::BadRequest("Wrong Auth header format".into()).error_response(),
        Some(auth) => auth
    };

    let cred = match identity_service::get_credential(auth, data).await{
        Ok(c) => c,
        Err(err) => return err.error_response()
    };

    HttpResponse::Ok().json(cred)
}

#[get("/is-credential-valid")]
pub async fn is_credential_valid(data: web::Data<AppState>, cred: web::Json<Credential>) -> HttpResponse{
    let res = match identity_service::is_credential_valid(cred.into_inner(), data).await{
        Ok(r) => r,
        Err(err) => return err.error_response()
    };
    HttpResponse::Ok().json(res)
}
