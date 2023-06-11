use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;

#[derive(Serialize)]
pub struct ServerError {
    pub error: String
}

#[derive(Serialize)]
pub struct ServerSuccess {
    pub message: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<Thing>,
    pub token: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub is_male: Option<bool>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicUser {
    pub id: Thing,
    pub username: String,
    pub is_male: Option<bool>
}