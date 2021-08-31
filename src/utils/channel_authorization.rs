use iota_identity_lib::iota::Credential;
use serde::{Serialize, Deserialize};
use actix_web::HttpRequest;
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DayTimestamp{
    day_timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelAuthorization {
    cred: Credential,
    channel_psw: String,
    day_timestamp: i64,
}

impl ChannelAuthorization {
    pub fn from_http_req_and_timestamp(req: HttpRequest, day_timestamp: &DayTimestamp) -> Option<Self>{
        let cred = req.headers().get("Cred")?.to_str().ok()?.to_owned();
        let cred = serde_json::from_str::<Credential>(&cred)
            .map_or(None, |c| Some(c))?;
        let channel_psw = req.headers().get("Channel-psw")?.to_str().ok()?.to_owned();
        let day_timestamp = day_timestamp.day_timestamp;
        Some(ChannelAuthorization{ cred, channel_psw, day_timestamp})
    }
    pub fn from_http_req_and_date(req: HttpRequest, date: &str) -> Option<Self>{
        let naive_date = NaiveDate::parse_from_str(date,"%d/%m/%Y")
            .map_or_else(|err| {
                eprintln!("{}", err.to_string());
                None
            },|date| Some(date))?;
        let timestamp = naive_date.and_hms_opt(0, 0, 0).unwrap().timestamp();
        let timestamp = DayTimestamp{day_timestamp: timestamp};
        ChannelAuthorization::from_http_req_and_timestamp(req, &timestamp)
    }
    pub fn cred(&self) -> &Credential {
        &self.cred
    }
    pub fn channel_psw(&self) -> &str {
        &self.channel_psw
    }
    pub fn day_timestamp(&self) -> i64 {
        self.day_timestamp
    }
}
