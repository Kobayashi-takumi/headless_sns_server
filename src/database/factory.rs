use anyhow::Result;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use crate::config::AppConfig;

pub async fn factory(config: &AppConfig) -> Result<Surreal<Client>> {
    let db = Surreal::new::<Ws>(config.db_path.clone()).await?;
    db.signin(Root {
        username: &config.db_user,
        password: &config.db_password,
    })
    .await?;
    db.use_ns(config.db_ns.clone())
        .use_db(config.db_name.clone())
        .await?;
    Ok(db)
}
