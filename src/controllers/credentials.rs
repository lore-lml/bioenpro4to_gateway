use actix_web::{web, get, post, HttpResponse, ResponseError as AWResponseError, Scope};
use crate::services::identity_service;
use crate::environment::AppState;
use crate::utils::AuthInfo;
use iota_identity_lib::iota::Credential;
use deadpool_postgres::Pool;
use crate::controllers::Controller;

pub struct CredentialController;
impl Controller for CredentialController{
    fn scope(scope_name: &str) -> Scope {
        web::scope(scope_name)
            .service(get_credential)
            .service(is_credential_valid)
    }
}

#[post("/authenticate")]
async fn get_credential(auth: web::Json<AuthInfo>, data: web::Data<AppState>, pool: web::Data<Pool>) -> HttpResponse{
    let cred = match identity_service::get_credential(auth.into_inner(), data, pool).await{
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
