use serde::{Deserialize, Serialize};
use aragorn::{AragornServiceError, Authenticate};

pub mod common;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub email: String,
    pub encrypted_password: String,
}

impl Authenticate for User {
    fn authenticate(&self, secret: &str) -> Result<(), AragornServiceError> {
        // Obviously you must use some bcrypt or Argon2 hashing tool
        if secret != self.encrypted_password {
            return Err(AragornServiceError::Unauthorized);
        }
        Ok(())
    }
}

#[test]
fn can_succeed() {
    let user = User {
        username: "MichelPolnareff4Ever".to_string(),
        email: "michou@gmail.net".to_string(),
        encrypted_password: "password".to_string()
    };
    user.authenticate("password").unwrap();
}

#[should_panic(expected = "Unauthorized")]
#[test]
fn can_fail() {
    let user = User {
        username: "MichelPolnareff4Ever".to_string(),
        email: "michou@gmail.net".to_string(),
        encrypted_password: "password".to_string()
    };
    user.authenticate("wrong").unwrap();
}