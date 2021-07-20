use actix_web::{web, post, get, HttpResponse, ResponseError};

use crate::services::streams_service;
use crate::environment::AppState;
use crate::utils::channels::ChannelCreationRequest;


#[post("/daily-channel")]
pub async fn create_daily_channel(body: web::Json<ChannelCreationRequest>,
                                  data: web::Data<AppState>) -> HttpResponse{
    let info = match streams_service::create_daily_channel(body.into_inner(), data).await{
        Ok(info) => info,
        Err(err) => return err.error_response()
    };
    HttpResponse::Ok().json(info)
}

#[get("/daily-channel")]
pub async fn get_daily_channel(body: web::Json<ChannelCreationRequest>,
                               data: web::Data<AppState>) ->  HttpResponse{
    let info = match streams_service::get_daily_channel(body.into_inner(), data).await{
        Ok(info) => info,
        Err(err) => return err.error_response()
    };
    HttpResponse::Ok().json(info)
}


