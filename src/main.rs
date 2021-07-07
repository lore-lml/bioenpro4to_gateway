mod services;

use actix_web::{web, get, App, HttpServer, Responder, HttpResponse};
use serde::Serialize;
use bioenpro4to_channel_manager::channels::root_channel::RootChannel;
use anyhow::Result;
use iota_identity_lib::api::IdentityManager;
use crate::services::AppState;
use crate::services::credentials::{get_credential, is_credential_valid};


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
    let mut root = RootChannel::new(false);
    let info = root.open("psw").await?;
    println!("Root Channel -> {}:{}", info.channel_id(), info.announce_id());

    let mut manager = IdentityManager::default().await?;
    let did = manager.create_identity("santer reply").await?.id().as_str().to_string();
    println!("Santer Reply DID: {}", did);

    let state = web::Data::new(AppState::new(root, manager));

    println!("Open at http://localhost:8080");

    HttpServer::new(move || {
        let credential_scope = web::scope("/id-manager")
            .service(get_credential)
            .service(is_credential_valid);
        App::new()
            .app_data(state.clone())
            .service(welcome)
            .service(hello)
            .service(credential_scope)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await?;
    Ok(())
}
