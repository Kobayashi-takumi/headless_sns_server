use anyhow::Result;
use serde::Deserialize;
use std::sync::Arc;
use surrealdb::{
    opt::IntoQuery,
    sql::statements::{BeginStatement, CancelStatement, CommitStatement},
    sql::Thing,
    Connection, Surreal,
};
use tokio::sync::Mutex;

pub mod post;

pub type DB<T> = Arc<Mutex<Surreal<T>>>;

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

///
/// It doesn't work because SurrealDB doesn't support transactions without embedded queries.
///
#[derive(Clone)]
pub struct Transaction<'a, T: Connection>(&'a DB<T>);

impl<'a, T: Connection> Transaction<'a, T> {
    pub fn new(db: &'a DB<T>) -> Self {
        Self(db)
    }
    pub async fn begin(&self) -> Result<&'a DB<T>> {
        self.query(BeginStatement).await?;
        Ok(&self.0)
    }
    pub async fn commit(&self) -> Result<()> {
        self.query(CommitStatement).await
    }
    pub async fn cancel(&self) -> Result<()> {
        self.query(CancelStatement).await
    }
    async fn query<S: IntoQuery>(&self, statement: S) -> Result<()> {
        self.0.lock().await.query(statement).await?;
        Ok(())
    }
}
