use actix_web::{
    web::{scope, Data},
    App, HttpServer,
};
use anyhow::Result;
use api::{service::post::PostFactory, ScopeFactory};
use config::AppConfig;
use database::factory::factory;
use dotenv::dotenv;
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};
use tokio::sync::Mutex;

mod adapter;
mod api;
mod config;
mod database;
mod util;

struct AppState {
    db: Arc<Mutex<Surreal<Client>>>,
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let config = AppConfig::new()?;
    let db = factory(&config).await?;
    HttpServer::new(move || {
        let db = db.clone();
        let app = App::new().app_data(Data::new(AppState {
            db: Arc::new(Mutex::new(db)),
        }));
        let app =
            app.service(scope(&PostFactory::scoped_name()).configure(PostFactory::scoped_config));
        // let app = app.service(echo);
        app
    })
    .bind((config.api_domain.clone(), config.api_port))?
    .run()
    .await?;
    Ok(())
}
