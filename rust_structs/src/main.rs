struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let mut user1 = new_user(
        String::from("rhunter"),
        String::from("roryhunter@gmail.com"),
    );

    user1.username = String::from("roryhunter1");

    let user2: User = User {
        username: String::from("user2name"),
        email: String::from("email.email.com"),
        ..user1
    };

    let user3 = new_user(String::from("rbh"), String::from("rbh@gmail.com"));

    println!(
        "user1 name:{}\nUser2 name:{}\nUser3 name:{}",
        user1.username, user2.username, user3.username
    );
}

fn new_user(username: String, email: String) -> User {
    let user = User {
        username: username,
        email: email,
        sign_in_count: 0,
        active: true,
    };
    user
}
