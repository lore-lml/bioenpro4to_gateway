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
    address: String,
    fiscal_code: String,
    phone_number: String,
    did: String,
    role: i32,
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
    pub fn did(&self) -> &str {
        &self.did
    }
    pub fn role(&self) -> i32 {
        self.role
    }
    pub fn address(&self) -> &str {
        &self.address
    }
    pub fn fiscal_code(&self) -> &str {
        &self.fiscal_code
    }
    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }
}
