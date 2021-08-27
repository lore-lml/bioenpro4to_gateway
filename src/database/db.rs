use actix_web::web;
use deadpool_postgres::Pool;
use crate::database::models::{User, Truck, Scale, BioCell};
use crate::errors::ResponseError;
use tokio_pg_mapper::FromTokioPostgresRow;

pub struct DBManager{
    pool: web::Data<Pool>,
}

impl DBManager{
    pub fn new(pool: web::Data<Pool>) -> Self {
        DBManager { pool }
    }
    pub async fn get_users(&self)-> Result<Vec<User>, ResponseError>{
        let client = match self.pool.get().await{
            Ok(c) => c,
            Err(_) => return Err(ResponseError::Internal("error during connection to database".into()))
        };

        let query = include_str!("scripts/get_users.sql");
        let query = query.replace("$table_fields", &User::sql_table_fields());
        let stmt = client.prepare(&query).await.unwrap();

        let users = client.query(&stmt, &[]).await.unwrap()
            .iter()
            .filter_map(|row| {
                match User::from_row_ref(row){
                    Ok(user) => Some(user),
                    Err(e) => {
                        eprintln!("{}", e.to_string());
                        None
                    }
                }
            })
            .collect();
        Ok(users)
    }

    pub async fn get_user(&self, user_id: &str) -> Result<User, ResponseError>{
        let client = match self.pool.get().await{
            Ok(c) => c,
            Err(_) => return Err(ResponseError::Internal("error during connection to database".into()))
        };

        let query = include_str!("scripts/get_user.sql");
        let query = query.replace("$table_fields", &User::sql_table_fields());
        let stmt = client.prepare(&query).await.unwrap();

        let row = match client.query_one(&stmt, &[&user_id.to_string()]).await {
            Ok(row) => row,
            Err(_) => return Err(ResponseError::NotFound(format!("User {} not found", user_id)))
        };
        match User::from_row_ref(&row){
            Ok(user) => Ok(user),
            Err(_) => Err(ResponseError::Internal("User row ref error".to_string()))
        }
    }

    pub async fn get_truck(&self, id: &str) -> Result<Truck, ResponseError>{
        let client = match self.pool.get().await{
            Ok(c) => c,
            Err(_) => return Err(ResponseError::Internal("error during connection to database".into()))
        };

        let query = include_str!("scripts/get_truck.sql");
        let query = query.replace("$table_fields", &Truck::sql_table_fields());
        let stmt = client.prepare(&query).await.unwrap();

        let row = match client.query_one(&stmt, &[&id.to_string()]).await {
            Ok(row) => row,
            Err(_) => return Err(ResponseError::NotFound(format!("Truck {} not found", id)))
        };
        match Truck::from_row_ref(&row){
            Ok(truck) => Ok(truck),
            Err(_) => Err(ResponseError::Internal("Truck row ref error".to_string()))
        }
    }

    pub async fn get_scale(&self, id: &str) -> Result<Scale, ResponseError>{
        let client = match self.pool.get().await{
            Ok(c) => c,
            Err(_) => return Err(ResponseError::Internal("error during connection to database".into()))
        };

        let query = include_str!("scripts/get_scale.sql");
        let query = query.replace("$table_fields", &Scale::sql_table_fields());
        let stmt = client.prepare(&query).await.unwrap();

        let row = match client.query_one(&stmt, &[&id.to_string()]).await {
            Ok(row) => row,
            Err(_) => return Err(ResponseError::NotFound(format!("Scale {} not found", id)))
        };
        match Scale::from_row_ref(&row){
            Ok(scale) => Ok(scale),
            Err(_) => Err(ResponseError::Internal("Scale row ref error".to_string()))
        }
    }

    pub async fn get_biocell(&self, id: &str) -> Result<BioCell, ResponseError>{
        let client = match self.pool.get().await{
            Ok(c) => c,
            Err(_) => return Err(ResponseError::Internal("error during connection to database".into()))
        };

        let query = include_str!("scripts/get_biocell.sql");
        let query = query.replace("$table_fields", &BioCell::sql_table_fields());
        let stmt = client.prepare(&query).await.unwrap();

        let row = match client.query_one(&stmt, &[&id.to_string()]).await {
            Ok(row) => row,
            Err(_) => return Err(ResponseError::NotFound(format!("BioCell {} not found", id)))
        };
        match BioCell::from_row_ref(&row){
            Ok(biocell) => Ok(biocell),
            Err(_) => Err(ResponseError::Internal("BioCell row ref error".to_string()))
        }
    }
}
