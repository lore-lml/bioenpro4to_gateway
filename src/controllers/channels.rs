use actix_web::{web, post, get, HttpResponse, ResponseError, Scope};

use crate::services::{streams_writer_service, streams_reader_service};
use crate::environment::AppState;
use crate::utils::channels::ChannelCreationRequest;
use crate::controllers::Controller;
use iota_identity_lib::iota::json;

pub struct ChannelController;
impl Controller for ChannelController{
    fn scope(scope_name: &str) -> Scope{
        web::scope(scope_name)
            .service(create_daily_channel)
            .service(get_daily_channel)
            .service(actors_of_category)
            .service(channels_of_actor)
            .service(messages_of_channel_of_actor)
    }
}

#[post("/daily-channel")]
async fn create_daily_channel(body: web::Json<ChannelCreationRequest>,
                                  data: web::Data<AppState>) -> HttpResponse{
    match streams_writer_service::create_daily_channel(body.into_inner(), data).await{
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => return err.error_response()
    }
}

#[get("/daily-channel")]
async fn get_daily_channel(body: web::Json<ChannelCreationRequest>,
                               data: web::Data<AppState>) ->  HttpResponse{
    let base64 = match streams_writer_service::get_daily_channel(body.into_inner(), data).await{
        Ok(info) => info,
        Err(err) => return err.error_response()
    };
    HttpResponse::Found().json(json!({
        "channel_base64": &base64
    }))
}

#[get("/categories/{category}/actors")]
async fn actors_of_category(category: web::Path<String>, state: web::Data<AppState>) -> HttpResponse{
    match streams_reader_service::actors_of_category(&category, state){
        Ok(actors) => HttpResponse::Found().json(&actors),
        Err(err) => return err.error_response()
    }
}

#[get("/categories/{category}/actors/{actor_id}")]
async fn channels_of_actor(params: web::Path<(String, String)>, state: web::Data<AppState>) -> HttpResponse{
    let (category, actor_id) = params.into_inner();
    match streams_reader_service::channels_of_actor(&category, &actor_id, state){
        Ok(daily_ch) => HttpResponse::Found().json(&daily_ch),
        Err(err) => return err.error_response()
    }
}

#[get("/categories/{category}/actors/{actor_id}/date/{date}")]
async fn messages_of_channel_of_actor(params: web::Path<(String, String, String)>, state: web::Data<AppState>) -> HttpResponse{
    let (category, actor_id, date) = params.into_inner();
    let date = date.replace("-", "/");
    match streams_reader_service::messages_of_channel_of_actor(&category, &actor_id, &date, state).await{
        Ok(msgs) => HttpResponse::Found().json(&msgs),
        Err(err) => return err.error_response()
    }
}
