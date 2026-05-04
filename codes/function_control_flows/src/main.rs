fn another_function() {
    println!("Another function!");
}

fn another_function_with_parameters(x: i32) {
    println!("x: {}", x);
}

fn another_function_with_return_value() -> i32 {
    5
}

// Statements and Expressions
// Statement
fn statement_example() {
    let _x = 5;
}

// Expression
fn expression_example() {
    let y = {
        let x = 5;
        x + 1
    };
    println!("y: {}", y);
}

fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_parameters(5);

    statement_example();
    expression_example();

    let result = another_function_with_return_value();
    println!("result: {}", result);

    // Control Flow
    let number = 3;
    if number < 5 {
        println!("number is less than 5");
    } else if number == 5 {
        println!("number is equal to 5");
    } else {
        println!("number is greater than 5");
    }

    // Loops
    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;
        }
    };
    println!("result: {}", result);

    // Loop with label
    'outer: loop {
        'inner: loop {
            break 'outer;
        }
    }

    // while
    let mut number = 0;
    while number < 5 {
        number += 1;
    }
    println!("number: {}", number);

    // for
    let arr = [1, 2, 3, 4, 5];
    for i in arr {
        println!("i: {}", i);
    }

    // for in reverse
    for i in (0..5).rev() {
        println!("i: {}", arr[i]);
    }
}
