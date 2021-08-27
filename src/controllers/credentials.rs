use actix_web::{web, get, HttpResponse, HttpRequest, ResponseError as AWResponseError, Scope};
use crate::services::identity_service;
use crate::environment::AppState;
use crate::utils::AuthInfo;
use crate::errors::ResponseError;
use iota_identity_lib::iota::Credential;
use iota_identity_lib::iota::json;
use deadpool_postgres::Pool;
use crate::controllers::Controller;

pub struct CredentialController;
impl Controller for CredentialController{
    fn scope(scope_name: &str) -> Scope {
        web::scope(scope_name)
            .service(actor_nonce_request)
            .service(get_credential)
            .service(is_credential_valid)
    }
}

#[get("/actor-nonce")]
async fn actor_nonce_request(req: HttpRequest, data: web::Data<AppState>, pool: web::Data<Pool>) -> HttpResponse{
    let auth = match AuthInfo::from_http_request(&req){
        None => return ResponseError::BadRequest("Wrong Auth header format".into()).error_response(),
        Some(auth) => auth
    };

    let nonce = match identity_service::get_nonce_for_actor(auth, data, pool).await{
        Ok(nonce) => nonce,
        Err(err) => return err.error_response()
    };
    HttpResponse::Ok().json(json!({"nonce": &nonce}))
}

#[get("/channel-credential")]
async fn get_credential(req: HttpRequest, data: web::Data<AppState>, pool: web::Data<Pool>) -> HttpResponse{
    let auth = match AuthInfo::from_http_request(&req){
        None => return ResponseError::BadRequest("Wrong Auth header format".into()).error_response(),
        Some(auth) => auth
    };

    let cred = match identity_service::get_credential(auth, data, pool).await{
        Ok(c) => c,
        Err(err) => return err.error_response()
    };

    HttpResponse::Ok().json(cred)
}

#[get("/is-credential-valid")]
async fn is_credential_valid(data: web::Data<AppState>, cred: web::Json<Credential>) -> HttpResponse{
    let res = match identity_service::is_credential_valid(cred.into_inner(), data).await{
        Ok(r) => r,
        Err(err) => return err.error_response()
    };
    HttpResponse::Ok().json(res)
}
