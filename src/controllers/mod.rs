use actix_web::Scope;

pub mod credentials;
pub mod channels;

pub trait Controller{
    fn scope(scope_name: &str) -> Scope;
}
