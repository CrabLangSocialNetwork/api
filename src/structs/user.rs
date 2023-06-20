use chrono::{DateTime, Utc};
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
pub struct PublicUser {
    username: String,
    is_male: Option<bool>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Default)]
pub struct PostAuthor {
    pub(crate) username: String,
    permission_level: PermissionLevel
}