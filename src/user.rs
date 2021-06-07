use serde::{Deserialize, Serialize};
use rand::distributions::Alphanumeric;
use rand::Rng;

/// A global User
///
/// This user is the secret sauce for all things to do with this API. There will be raw counterparts
/// but for now this is what you need.
///
/// Implements all the serde types and provides a random implementation to provide quick users.
///
/// id: the id of the email
/// domain: email domain
/// password: password
/// email_token: the jwt token returned by mail-tm
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub domain: String,
    pub password: String,
    pub email_token: String,
}

impl User {
    pub fn new(email: &str, password: &str, domain: &str) -> User {
        User {
            id: email.to_string(),
            domain: domain.to_string(),
            password: password.to_string(),
            email_token: "".to_string(),
        }
    }

    pub fn with_domain(self, domain: &str) -> User {
        User {
            domain: domain.to_string(),
            ..self
        }
    }

    fn get_random_string(len: usize) -> String {
        rand::thread_rng().sample_iter(&Alphanumeric).take(len).map(char::from).collect()
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            id: User::get_random_string(10), // Default could be rand
            password: User::get_random_string(13),
            email_token: "".to_string(),
            domain: "".to_string(),
        }
    }
}
