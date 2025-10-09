use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
}
