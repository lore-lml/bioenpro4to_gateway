mod services;
mod environment;

use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use anyhow::Result;
use serde::Serialize;
use iota_identity_lib::iota::json;

use crate::services::channels::{create_daily_channel, get_daily_channel};
use crate::services::credentials::{get_credential, is_credential_valid};
use crate::environment::{EnvConfig, AppState};


#[derive(Serialize)]
pub struct Message{
    msg: String,
}

#[get("/")]
async fn welcome(state: web::Data<AppState>) -> impl Responder{
    let addr = {
        let info = state.root.lock().unwrap().channel_info();
        format!("{}:{}", info.channel_id(), info.announce_id())
    };
    let issuer_name = state.config.identity_issuer_name();
    let issuer_did = {
        let manager = state.identity.lock().unwrap();
        manager.get_identity(issuer_name).unwrap().id().as_str().to_string()
    };
    let json = json!({
        "message": "Welcome to BioEnPro4To gateway",
        "issuer": issuer_name,
        "issuer_did": &issuer_did,
        "channel_address": &addr,
    });
    HttpResponse::Ok().json(json)
}

#[actix_web::main]
async fn main() -> Result<()> {
    let config = EnvConfig::from_env()?;
    let url = config.url();
    let binding_address = config.address();

    let state = web::Data::new(AppState::from_config(config).await?);
    println!("Open at {}", url);

    HttpServer::new(move || {
        let credential_scope = web::scope("/id-manager")
            .service(get_credential)
            .service(is_credential_valid);
        let channels_scope = web::scope("/channel-manager")
            .service(create_daily_channel)
            .service(get_daily_channel);
        App::new()
            .app_data(state.clone())
            .service(welcome)
            .service(credential_scope)
            .service(channels_scope)
    })
        .bind(binding_address)?
        .run()
        .await?;

    Ok(())
}
