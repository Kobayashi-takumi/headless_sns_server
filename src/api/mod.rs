use actix_web::web::ServiceConfig;

pub mod service;

pub trait ScopeFactory {
    fn scoped_config(cfg: &mut ServiceConfig);
    fn scoped_name() -> String;
}
