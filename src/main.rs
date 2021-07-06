use actix_web::{web, get, App, HttpServer, Responder, HttpResponse};
use serde::Serialize;
use bioenpro4to_channel_manager::channels::root_channel::RootChannel;
use anyhow::Result;
use std::sync::Mutex;

pub struct AppState{
    root: Mutex<RootChannel>
}

#[derive(Serialize)]
pub struct Message{
    msg: String,
}

#[get("/{name}")]
async fn hello(web::Path(name): web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    if name.starts_with("l"){
        let root = data.root.lock().unwrap();
        let info = format!("{:#?}", root.channel_info());
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
    println!("Open at http://localhost:8080");
    let state = web::Data::new(AppState{root: Mutex::new(root)});
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(welcome)
            .service(hello)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await?;
    Ok(())
}
