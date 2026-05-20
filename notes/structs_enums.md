# Structs and Enums

## Structs

**Definition:** It is a custom datatype that lets you define a new type with named fields.

### Defining and Instantiating Structs

```rust
struct Person {
    name: String,
    age: u32,
}

let person = Person {
    name: String::from("Alice"),
    age: 30,
};
```

### Using the Field Init Shorthand

It allows you to omit the field name when initializing a struct. Similar to JavaScript object literal syntax.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

### Constructing Structs from Existing Structs

- The `..` syntax allows you to use the remaining fields from an existing struct.

```rust
// refer main.rs for the `create_user` function
let user3 = create_user(String::from("charlie@example.com"), String::from("charlie"));

// Creating a user from an existing user
let user4 = User {
    email: String::from("don@example.com"),
    username: String::from("don"),
    ..user3
};
```

## Enums
