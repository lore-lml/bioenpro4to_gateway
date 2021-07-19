use tokio_pg_mapper_derive::PostgresMapper;
use serde::{Serialize, Deserialize};

#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table="bioenpro4to.users")]
pub struct User{
    id: String,
    email: String,
    first_name: String,
    last_name: String,
    did: Option<String>,
}
