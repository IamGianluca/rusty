use std::collections::HashMap;

use crate::user::User;

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
            Some(user) => {
                return Ok(Client {
                    user: User {
                        name: user.to_string(),
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
        assert_eq!(response.is_ok(), true);
        let client = response.unwrap();
        assert_eq!(client.user.name, "luca")
    }

    #[test]
    fn test_login_fail_wrong_token() {
        let response = Client::new("wrong_token".to_string());
        assert_eq!(response.is_ok(), false);
    }
}
