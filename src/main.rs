mod services;
mod environment;

use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use anyhow::Result;
use serde::Serialize;

use crate::services::channels::create_daily_channel;
use crate::services::credentials::{get_credential, is_credential_valid};
use crate::environment::{EnvConfig, AppState};


#[derive(Serialize)]
pub struct Message{
    msg: String,
}

#[get("/")]
async fn welcome() -> impl Responder{
    HttpResponse::Ok().body("Welcome to BioEnPro4To gateway")
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
            .service(is_credential_valid)
            .service(create_daily_channel);
        App::new()
            .app_data(state.clone())
            .service(welcome)
            .service(credential_scope)
    })
        .bind(binding_address)?
        .run()
        .await?;

    Ok(())
}
