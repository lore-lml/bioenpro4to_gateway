use actix_web::web;
use deadpool_postgres::Pool;
use crate::database::models::User;
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
            Err(_) => Err(ResponseError::NotFound(format!("User {} not found", user_id)))
        }
    }
}
