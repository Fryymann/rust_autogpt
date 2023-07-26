
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


#[cfg(test)]
    use super::*;

    #[test]
    fn test_structs() {
        let user_1: User = User {
            username: String::from("someone"),
            email: String::from("someone@somewhere.com"),
            active: true,
            sign_in_count: 1,
        };

        dbg!(user_1);
    }