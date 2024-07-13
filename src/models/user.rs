use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Deserialize, Serialize)]
pub struct AddUserRequest {
    #[validate(length(min = 1, message = "username required"))]
    pub username: String
}

#[derive(Validate, Deserialize, Serialize)]
pub struct UpdateUrl {
    pub uuid: String
}

#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct User {
    pub uuid: String,
    pub username: String
}

impl User {
    pub fn new(uuid: String, username: String) -> User {
        User { uuid, username }
    }
}