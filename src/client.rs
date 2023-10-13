use std::collections::HashMap;

use chrono::Utc;

use crate::domain::user::User;

pub struct TokenUserTable {
    token2username: HashMap<String, String>,
}
impl TokenUserTable {
    pub fn new() -> TokenUserTable {
        TokenUserTable {
            token2username: HashMap::new(),
        }
    }
}

pub struct Client {
    user: User,
}
impl Client {
    pub fn new(token: String) -> Result<Client, String> {
        let mut table = TokenUserTable::new();
        table
            .token2username
            .insert("token".to_string(), "luca".to_string());

        match &table.token2username.get(&token.to_string()) {
            None => return Err("Wrong access token".to_string()),
            Some(_) => {
                return Ok(Client {
                    user: User {
                        id: 1,
                        username: "John Doe".to_string(),
                        email: "johndoe@example.com".to_string(),
                        created_at: Utc::now(),
                    },
                })
            }
        };
    }
}

#[cfg(test)]
mod test {

    use crate::client::Client;

    #[test]
    fn test_login() {
        let response = Client::new("token".to_string());
        assert!(response.is_ok());
        let client = response.unwrap();
        assert_eq!(client.user.username, "John Doe")
    }

    #[test]
    fn test_login_fail_wrong_token() {
        let response = Client::new("wrong_token".to_string());
        assert_eq!(response.is_ok(), false);
    }
}
