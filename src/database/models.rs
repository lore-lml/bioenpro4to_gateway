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

#[allow(dead_code)]
impl Truck{
    pub fn plate(&self) -> &str {
        &self.plate
    }
    pub fn did(&self) -> &str {
        &self.did
    }
    pub fn driver(&self) -> &str {
        &self.driver
    }
}

#[allow(dead_code)]
impl Scale{
    pub fn plant(&self) -> &str {
        &self.plant
    }
    pub fn did(&self) -> &str {
        &self.did
    }
}

#[allow(dead_code)]
impl BioCell{
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn plant(&self) -> &str {
        &self.plant
    }
    pub fn max_capacity(&self) -> i32 {
        self.max_capacity
    }
    pub fn did(&self) -> &str {
        &self.did
    }
}
