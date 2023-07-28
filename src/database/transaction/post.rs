use crate::database::dto::Dto;

use super::{super::dto::post::Post, Record, DB};
use anyhow::{anyhow, Result};
use surrealdb::Connection;

pub async fn create<C: Connection>(db: &DB<C>, value: Post) -> Result<()> {
    let mut res = db
        .lock()
        .await
        .query(
            r#"
UPDATE 
    $id
CONTENT
    {{
        content: $content,
        hash_tags: $hash_tags,
        mentions: $mentions,
        created_at: $created_at,
        updated_at: $updated_at        
    }};
            "#,
        )
        .bind(("id", value.id))
        .bind(("content", value.content.as_str()))
        .bind(("hash_tags", &value.hash_tags))
        .bind(("mentions", &value.mentions))
        .bind(("created_at", value.created_at))
        .bind(("updated_at", value.updated_at))
        .await?;
    let res: Option<Record> = res.take(0)?;
    println!("{res:?}");
    Ok(())
}

pub async fn get<C: Connection>(db: &DB<C>, value: String) -> Result<Post> {
    let res: Option<Post> = db.lock().await.select((Post::table(), value)).await?;
    match res {
        Some(val) => Ok(val),
        _ => Err(anyhow!("Not Found")),
    }
}

pub async fn list<C: Connection>(db: &DB<C>) -> Result<Vec<Post>> {
    let mut res = db
        .lock()
        .await
        .query(
            r#"
SELECT
    id,
    content,
    hash_tags,
    mentions,
    created_at,
    updated_at
FROM
    type::table($tb)
ORDER BY
    updated_at DESC;
        "#,
        )
        .bind(("tb", Post::table()))
        .await?;
    let res: Vec<Post> = res.take(0)?;
    Ok(res)
}
