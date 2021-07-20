mod controllers;
mod environment;
mod database;
mod utils;
mod services;
mod errors;

use actix_web::{App, get, HttpResponse, HttpServer, Responder, web, ResponseError};
use anyhow::Result;
use serde::Serialize;
use iota_identity_lib::iota::json;

use crate::controllers::channels::{create_daily_channel, get_daily_channel};
use crate::controllers::credentials::{get_credential, is_credential_valid};
use crate::environment::{EnvConfig, AppState};
use crate::database::DbConfig;
use deadpool_postgres::Pool;

use crate::database::models::User;
use crate::database::db;
use crate::database::db::DBManager;
use std::future::Future;


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
    let db_manager = DBManager::new(pool);
    let users = match db_manager.get_users().await{
        Ok(users) => users,
        Err(err) => return err.error_response()
    };
    HttpResponse::Ok().json(users)
}

#[get("/users/{user_id}")]
async fn user(user_id: web::Path<String>, pool: web::Data<Pool>) -> impl Responder{
    let db_manager = DBManager::new(pool);
    let user = match db_manager.get_user(user_id.into_inner()).await{
        Ok(user) => user,
        Err(err) => return err.error_response()
    };
    HttpResponse::Ok().json(user)
}


#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let mut db_config = DbConfig::from_env()?;
    let pool = web::Data::new(db_config.create_pool()?);
    //Test database connection
    let client = pool.get().await?;
    drop(client);

    let env_config = EnvConfig::from_env()?;
    let url = env_config.url();
    let binding_address = env_config.address();

    let state = web::Data::new(AppState::from_config(env_config).await?);

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
            .app_data(pool.clone())
            .service(welcome)
            .service(users)
            .service(user)
            .service(credential_scope)
            .service(channels_scope)
    })
        .bind(binding_address)?
        .run()
        .await?;
    Ok(())
}
