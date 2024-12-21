use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub email: String,
}

#[derive(Debug)]
pub struct SecretFriend {
    pub person: Person,
    pub secret_friend: Person,
}
