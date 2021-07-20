mod controllers;
mod environment;
mod database;
mod utils;
mod services;
mod errors;

use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use anyhow::Result;
use serde::Serialize;
use iota_identity_lib::iota::json;

use crate::controllers::channels::{create_daily_channel, get_daily_channel};
use crate::controllers::credentials::{get_credential, is_credential_valid};
use crate::environment::{EnvConfig, AppState};
use crate::database::DbConfig;
use deadpool_postgres::Pool;
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::database::db::User;
use bioenpro4to_channel_manager::utils::hash_string;


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

#[get("/users")]
async fn users(pool: web::Data<Pool>) -> impl Responder{
    let client = match pool.get().await{
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("error during connection to database")
    };

    let query = "SELECT * FROM bioenpro4to.users";
    let stmt = client.prepare(query).await.unwrap();
    let users: Vec<User> = client.query(&stmt, &[]).await.unwrap()
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect();

    HttpResponse::Ok().json(users)
}


#[actix_web::main]
async fn main() -> Result<()> {
    // dotenv::dotenv().ok();
    // let mut db_config = DbConfig::from_env()?;
    // let pool = web::Data::new(db_config.create_pool()?);
    //
    // let env_config = EnvConfig::from_env()?;
    // let url = env_config.url();
    // let binding_address = env_config.address();
    //
    // let state = web::Data::new(AppState::from_config(env_config).await?);
    //
    // println!("Open at {}", url);
    //
    // HttpServer::new(move || {
    //     let credential_scope = web::scope("/id-manager")
    //         .service(get_credential)
    //         .service(is_credential_valid);
    //     let channels_scope = web::scope("/channel-manager")
    //         .service(create_daily_channel)
    //         .service(get_daily_channel);
    //     App::new()
    //         .app_data(state.clone())
    //         .app_data(pool.clone())
    //         .service(welcome)
    //         .service(users)
    //         .service(credential_scope)
    //         .service(channels_scope)
    // })
    //     .bind(binding_address)?
    //     .run()
    //     .await?;

    println!("{}", hash_string("psw"));
    Ok(())
}
