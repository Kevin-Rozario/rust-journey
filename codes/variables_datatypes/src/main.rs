fn main() {
    // Variables
    let mut x = 5; // Note: After this line, x is mutable upon adding `mut`.
    println!("The value of x: {x}");

    x = 6; // Panic: reassignment to immutable variable
    println!("The value of x: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("Inner x: {x}"); // 12
    }

    println!("Outer x: {x}"); // 6

    // let mut spaces = "   ";
    // spaces = spaces.len(); // Panic: cannot assign to `spaces` because it is a `let` binding

    // Data Types
    // Scalar
    let i: i32 = 42;
    let f: f64 = 3.14;
    let b: bool = true;
    let c: char = 'a';

    println!("i: {i}, f: {f}, b: {b}, c: {c}");

    // Compound
    // Tuple
    let t: (i32, f64, bool, char) = (42, 3.14, true, 'a');
    println!("t: {t:?}");

    // Tuple unpacking
    let (x, y, z, w) = t;
    println!("x: {x}, y: {y}, z: {z}, w: {w}");

    // Tuple indexing
    let x = t.0;
    let y = t.1;
    let z = t.2;
    let w = t.3;
    println!("x: {x}, y: {y}, z: {z}, w: {w}");

    // Array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: {a:?}");

    // Array indexing
    let x = a[0];
    let y = a[1];
    let z = a[2];
    let w = a[3];
    let v = a[4];
    println!("x: {x}, y: {y}, z: {z}, w: {w}, v: {v}");
}
