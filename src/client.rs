use std::collections::HashMap;

use crate::user::User;

pub struct TokenUserTable {
    ha: HashMap<String, String>,
}
impl TokenUserTable {
    pub fn new() -> TokenUserTable {
        let table = TokenUserTable { ha: HashMap::new() };
        table
    }
}

pub struct Client {
    user: User,
}
impl Client {
    pub fn new(token: String) -> Client {
        let mut table = TokenUserTable::new();
        table.ha.insert("token".to_string(), "luca".to_string());
        let user = &table.ha[&token.to_string()];
        Client {
            user: User {
                name: user.to_string(),
            },
        }
    }
}

#[cfg(test)]
mod test {

    use crate::client::Client;

    #[test]
    fn test_login() {
        let client = Client::new("token".to_string());
        assert_eq!(client.user.name, "luca")
    }
}
