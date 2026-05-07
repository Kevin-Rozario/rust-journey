fn main() {
    // Strings
    let _s1 = "This is a string literal stored on the stack.";
    let mut s2 = String::from("This is a string literal stored on the heap");

    // Mutable s2
    s2.push_str(" and is mutable.");
    println!("{s2}");

    // Memory and Allocation
    // Variable and Data interacting with "Move"
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y} (Forms two copies on the stack with known size)");

    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    println!("s2 = {s2})");

    // Varaible and Data interacting with "Clone"
    let s1 = String::from("something");
    let s2 = s1.clone(); // s1 is cloned to s2, s1 is still valid
    println!("s1 = {s1}, s2 = {s2}");

    // Ownership and Functions
    let s = String::from("I don't own you anymore!");
    takes_ownership(s); // s is moved to the function, s is no longer valid

    let x = 1245;
    makes_copy(x); // x is copied to the function, x is still valid

    let mut s4 = String::from("I still own you!");
    s4 = takes_gives_back_ownership(s4); // s3 is moved to the function, s3 is no longer valid
    println!("s4 = {s4}");

    // References and Borrowing
    let refer_string = String::from("I am a reference to a string stored on the heap.");
    println!("Length: {}", calculate_length(&refer_string)); // refer_string is borrowed its still in scope and valid

    let mut refer_string2 = String::from("Hello");
    println!("Before mutation: {refer_string2}");
    mutate_string(&mut refer_string2);
    println!("After mutation: {refer_string2}");

    // Mutable References
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("r3 = {r3}");

    // Dangling References
    // let reference_to_nothing = dangle(); // this will cause a compile error

    // Slices
    // String Slices
    let s = String::from("Hello, World!");
    let slice = &s[0..5];
    println!("slice = {slice}");

    // Array Slices
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[0..3];
    println!("slice = {slice:?}");
}

fn takes_ownership(s: String) {
    println!("s = {s}");
}

fn makes_copy(x: i32) {
    println!("x = {x}");
}

fn takes_gives_back_ownership(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn mutate_string(s: &mut String) {
    s.push_str(", World!");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
