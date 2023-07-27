
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn increment_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }

    fn change_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }

    fn change_username(&mut self, new_username: &str) {
        self.username = String::from(new_username);
    }
}


// fn change_username(user: &mut User, new_username: &str ) {
//     user.username = String::from(new_username);
// }


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

        let mut user_2: User = User {
            username: String::from("someone2"),
            email: String::from("someone2@somewhere.com"),
            active: false,
            sign_in_count: 0,
        };

        user_2.increment_sign_in_count();
        user_2.change_email("newemail@user2.com");

        dbg!(user_2);



    }