mod services;

use actix_web::{web, get, App, HttpServer, Responder, HttpResponse};
use serde::Serialize;
use anyhow::Result;
use crate::services::{AppState, EnvConfig};
use crate::services::credentials::{get_credential, is_credential_valid};
use crate::services::channels::create_daily_channel;

#[derive(Serialize)]
pub struct Message{
    msg: String,
}

#[get("/{name}")]
async fn hello(name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    if name.starts_with("l"){
        let root = data.root.lock().unwrap();
        let did = data.identity.lock().unwrap()
            .get_identity("santer reply").unwrap().id().to_string();
        let info = format!("{:#?}\n{}", root.channel_info(), did);
        return HttpResponse::Unauthorized().body(info);
    }
    HttpResponse::Ok().json(Message{msg: format!("Hello {}!", name)})
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
            .service(hello)
            .service(credential_scope)
    })
        .bind(binding_address)?
        .run()
        .await?;

    Ok(())
}
