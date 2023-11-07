struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = build_user(String::from("email@example.com"), String::from("fakeUser"));

    println!("{}", user1.email);

    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{}", user2.email);
    println!("{}", user1.active);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}