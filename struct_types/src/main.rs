struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String,  email: String) -> User {
    User{
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("Abishek Bista"),
        email: String::from("abishekbista84@gmail.com"),
        sign_in_count: 1,
    };

    let user2 = build_user(String::from("ana"), String::from("ana@gmail.com"));

    let user3 = User {
        email: String::from("anaschmidt@gmail.com"),
        ..user1
    };

    println!("User info::");
    println!("Active: {}", user2.active);
    println!("Name: {}", user2.username);
    println!("Email: {}", user2.email);
    println!("Sign in count: {}", user2.sign_in_count);
}