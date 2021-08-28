use tokio_pg_mapper_derive::PostgresMapper;
use serde::{Serialize, Deserialize};
use bioenpro4to_channel_manager::channels::Category;

#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table="bioenpro4to.actors")]
pub struct Actor{
    id: String,
    #[serde(skip_serializing)]
    psw: String,
    did: String,
    category: i32,
}

#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table="bioenpro4to.trucks")]
pub struct Truck{
    plate: String,
    driver: String
}

#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table="bioenpro4to.scales")]
pub struct Scale{
    plant: String,
}

#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table="bioenpro4to.biocells")]
pub struct BioCell{
    digestor_id: String,
    plant: String,
    max_capacity: i32,
}

#[allow(dead_code)]
impl Actor{
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn psw(&self) -> &str {
        &self.psw
    }
    pub fn did(&self) -> &str {
        &self.did
    }
    pub fn category(&self) -> Category {
        match self.category{
            0 => Category::Trucks,
            1 => Category::Scales,
            _ => Category::BioCells,
        }
    }
}

#[allow(dead_code)]
impl Truck{
    pub fn plate(&self) -> &str {
        &self.plate
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
}

#[allow(dead_code)]
impl BioCell{
    pub fn digestor_id(&self) -> &str {
        &self.digestor_id
    }
    pub fn plant(&self) -> &str {
        &self.plant
    }
    pub fn max_capacity(&self) -> i32 {
        self.max_capacity
    }
}
