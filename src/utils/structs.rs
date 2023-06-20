use serde::{Serialize, Deserialize};
use surrealdb::sql::{Thing, Datetime};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, PartialOrd)]
pub enum PermissionLevel {
    #[default]
    Guest,
    User,
    Moderator,
    Administrator
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub(crate) id: Option<Thing>,
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) password: Vec<u8>,
    pub(crate) is_male: Option<bool>,
    pub(crate) token: String,
    pub(crate) permission_level: PermissionLevel,
    pub(crate) created_at: Datetime,
    pub(crate) updated_at: Datetime
}

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

#[derive(Serialize, Deserialize, Default)]
pub struct PostAuthor {
    pub(crate) username: String,
    permission_level: PermissionLevel
}

#[derive(Serialize, Deserialize)]
pub struct PublicUser {
    username: String,
    is_male: Option<bool>,
    created_at: Datetime,
    updated_at: Datetime
}