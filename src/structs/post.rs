use serde::{Serialize, Deserialize};
use surrealdb::sql::{Thing, Datetime};

use super::user::PostAuthor;

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub(crate) id: Option<Thing>,
    pub(crate) content: String,
    pub(crate) images: Vec<String>,
    pub(crate) author: Thing,
    pub(crate) created_at: Datetime,
    pub(crate) updated_at: Datetime
}

#[derive(Deserialize, Serialize)]
pub struct PublicPost {
    id: String,
    content: String,
    //vector of images links
    images: Vec<String>,
    pub(crate) author: PostAuthor,
    #[serde(default)] pub(crate) has_permission: bool,
    created_at: Datetime,
    updated_at: Datetime
}