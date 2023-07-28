use crate::api::ScopeFactory;
use crate::database::transaction::post::{create, get as get_tran, list as list_tran};
use crate::AppState;
use actix_web::{
    web::{self, resource, Data, Json, Path},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub content: String,
    pub hash_tags: Vec<String>,
    pub mentions: Vec<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Posts {
    pub items: Vec<Post>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct PostForm {
    pub content: String,
    pub hash_tags: Vec<String>,
    pub mentions: Vec<String>,
}

async fn post(body: Json<PostForm>, state: Data<AppState>) -> impl Responder {
    println!("{body:?}");
    let req = body.into_inner();
    match create(&state.db, req.into()).await {
        Ok(_) => HttpResponse::Ok().body("OK"),
        Err(e) => HttpResponse::BadRequest().body(format!("{e:?}").to_string()),
    }
}

async fn get(path: Path<String>, state: Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    println!("{id}");
    match get_tran(&state.db, id).await {
        Ok(res) => HttpResponse::Ok().json(Post::from(res)),
        Err(e) => HttpResponse::BadRequest().body(format!("{e:?}").to_string()),
    }
}

async fn list(state: Data<AppState>) -> impl Responder {
    match list_tran(&state.db).await {
        Ok(res) => HttpResponse::Ok().json(Posts::from(res)),
        Err(e) => HttpResponse::BadRequest().body(format!("{e:?}").to_string()),
    }
}

pub struct PostFactory;
impl ScopeFactory for PostFactory {
    fn scoped_config(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            resource("")
                .route(web::post().to(post))
                .route(web::get().to(list)),
        )
        .service(resource("/{id}").route(web::get().to(get)));
    }
    fn scoped_name() -> String {
        "/posts".to_string()
    }
}
