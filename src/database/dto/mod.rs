use chrono::NaiveDateTime;
use surrealdb::sql::Thing;
use uuid::Uuid;

pub mod post;

pub trait Dto {
    fn id() -> Thing {
        (Self::table(), Uuid::new_v4().to_string()).into()
    }
    fn table() -> String;
}
