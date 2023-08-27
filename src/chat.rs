struct Chat {
    namespace: String,
}

#[cfg(test)]
mod test {

    use crate::chat::Chat;

    #[test]
    fn test_create_chat() {
        let chat1 = Chat {
            namespace: String::from("company1"),
        };
        assert_eq!(chat1.namespace, "company1");
    }
}
