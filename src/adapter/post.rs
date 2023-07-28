use crate::api::service::post::{Post, PostForm, Posts};
use crate::database::dto::post::Post as PostDto;
use std::convert::From;

impl From<PostForm> for PostDto {
    fn from(value: PostForm) -> Self {
        Self::new(value.content, value.hash_tags, value.mentions)
    }
}

impl From<PostDto> for Post {
    fn from(value: PostDto) -> Self {
        Self {
            id: value.id.id.to_raw(),
            content: value.content,
            hash_tags: value.hash_tags,
            mentions: value.mentions,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<Vec<PostDto>> for Posts {
    fn from(value: Vec<PostDto>) -> Self {
        Self {
            items: value.into_iter().map(|i| i.into()).collect(),
        }
    }
}
