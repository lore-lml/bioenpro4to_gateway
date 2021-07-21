use tokio_pg_mapper_derive::PostgresMapper;
use serde::{Serialize, Deserialize};

#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table="bioenpro4to.roles")]
pub struct Role{
    id: i32,
    role: String,
}

#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table="bioenpro4to.users")]
pub struct User{
    id: String,
    #[serde(skip_serializing)]
    psw: String,
    email: String,
    first_name: String,
    last_name: String,
    did: Option<String>,
    role: i32,
}

#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table="bioenpro4to.trucks")]
pub struct Truck{
    plate: String,
    did: String,
    driver: String
}

#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table="bioenpro4to.scales")]
pub struct Scale{
    plant: String,
    did: String,
}

#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table="bioenpro4to.biocells")]
pub struct BioCell{
    id: String,
    plant: String,
    max_capacity: i32,
    did: String,
}

#[allow(dead_code)]
impl User{
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn psw(&self) -> &str {
        &self.psw
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn first_name(&self) -> &str {
        &self.first_name
    }
    pub fn last_name(&self) -> &str {
        &self.last_name
    }
    pub fn did(&self) -> Option<String> {
        self.did.clone()
    }
    pub fn role(&self) -> i32 {
        self.role
    }
}