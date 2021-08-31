use actix_web::{web, post, get, HttpResponse, Scope, HttpRequest, ResponseError as AWResponseError};

use crate::services::{streams_writer_service, streams_reader_service};
use crate::environment::AppState;
use crate::utils::channel_authorization::{ChannelAuthorization, DayTimestamp};
use crate::controllers::Controller;
use iota_identity_lib::iota::json;
use std::collections::HashMap;
use serde_json::Value;
use deadpool_postgres::Pool;
use crate::errors::ResponseError;
use std::ops::Deref;
use crate::utils::match_and_map_date_format;

pub struct ChannelController;
impl Controller for ChannelController{
    fn scope(scope_name: &str) -> Scope{
        web::scope(scope_name)
            .service(create_daily_channel)
            .service(get_daily_channel)
            .service(actors_of_category)
            .service(channels_of_actor)
            .service(messages_of_channel_of_actor)
            .service(actors_last_updates)
    }
}

#[post("/daily-channel")]
async fn create_daily_channel(req: HttpRequest,
                              timestamp: web::Json<DayTimestamp>,
                              data: web::Data<AppState>,
                              pool: web::Data<Pool>) -> HttpResponse{
    let ch_auth = match ChannelAuthorization::from_http_req_and_timestamp(req, timestamp.deref()){
        None => return ResponseError::BadRequest("Wrong Auth header format".into()).error_response(),
        Some(auth) => auth
    };
    match streams_writer_service::create_daily_channel(ch_auth, data, pool).await{
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => return err.error_response()
    }
}

#[get("/daily-channel/{date}")]
async fn get_daily_channel(req: HttpRequest,
                           date: web::Path<String>,
                           data: web::Data<AppState>,
                           pool: web::Data<Pool>) ->  HttpResponse{
    let date = match match_and_map_date_format(date.deref()){
        Ok(date) => date,
        Err(err) => return err.error_response()
    };
    let ch_auth = match ChannelAuthorization::from_http_req_and_date(req, &date){
        None => return ResponseError::BadRequest("Wrong Auth header format".into()).error_response(),
        Some(auth) => auth
    };
    let base64 = match streams_writer_service::get_daily_channel(ch_auth, data, pool).await{
        Ok(info) => info,
        Err(err) => return err.error_response()
    };
    HttpResponse::Ok().json(json!({
        "channel_base64": &base64
    }))
}

#[get("/categories/{category}/actors")]
async fn actors_of_category(category: web::Path<String>, state: web::Data<AppState>) -> HttpResponse{
    match streams_reader_service::actors_of_category(&category, state){
        Ok(actors) => HttpResponse::Ok().json(&actors),
        Err(err) => return err.error_response()
    }
}

#[get("/categories/{category}/actors/{actor_id}")]
async fn channels_of_actor(params: web::Path<(String, String)>, state: web::Data<AppState>) -> HttpResponse{
    let (category, actor_id) = params.into_inner();
    match streams_reader_service::channels_of_actor(&category, &actor_id, state){
        Ok(daily_ch) => HttpResponse::Ok().json(&daily_ch),
        Err(err) => return err.error_response()
    }
}

#[get("/categories/{category}/actors/{actor_id}/date/{date}")]
async fn messages_of_channel_of_actor(params: web::Path<(String, String, String)>, state: web::Data<AppState>) -> HttpResponse{
    let (category, actor_id, date) = params.into_inner();
    match streams_reader_service::messages_of_channel_of_actor(&category, &actor_id, &date, state).await{
        Ok(msgs) => HttpResponse::Ok().json(&msgs),
        Err(err) => return err.error_response()
    }
}

#[get("/actors-last-updates/{count}")]
async fn actors_last_updates(count: web::Path<String>, state: web::Data<AppState>) -> HttpResponse{
    let count: u16 = match count.parse(){
        Ok(count) => count,
        Err(_) => return HttpResponse::BadRequest().body("Invalid parameter format last-updates/<count>")
    };

    if count == 0{
        let empty: Vec<HashMap<String, Value>> = vec![];
        return HttpResponse::Ok().json(empty);
    }

    let updates = match streams_reader_service::actors_last_updates(count, state).await{
        Ok(updates) => updates,
        Err(err) => return err.error_response()
    };
    HttpResponse::Ok().json(updates)
}
