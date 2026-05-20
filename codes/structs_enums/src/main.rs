struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn create_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        active: false,
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        sign_in_count: 0,
    };

    println!(
        "User 1: active={}, username={}, email={}, sign_in_count={}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    user2.email = String::from("bob234@example.com"); //Note: user2 is mutable, so the email can be changed
    println!(
        "User 2: active={}, username={}, email={}, sign_in_count={}",
        user2.active, user2.username, user2.email, user2.sign_in_count
    );

    let user3 = create_user(String::from("charlie@example.com"), String::from("charlie"));
    println!(
        "User 3: active={}, username={}, email={}, sign_in_count={}",
        user3.active, user3.username, user3.email, user3.sign_in_count
    );

    // Creating a user from an existing user
    let user4 = User {
        email: String::from("don@example.com"),
        username: String::from("don"),
        ..user3
    };
    println!(
        "User 4: active={}, username={}, email={}, sign_in_count={}",
        user4.active, user4.username, user4.email, user4.sign_in_count
    );
}
