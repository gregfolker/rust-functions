// Project: functions
// Author: Greg Folker

fn main() {
	println!("Hello, World!");

    another_function(5, 6);

    // Expressions evaluate to something, for example '5 + 6' is an expression that evaluates to 11
    // Calling a function is considered to be an expression
    // Calling a macro is an expression
    // The values inside of the block {} are local to that scope
    // This can be thought of as an inline function
    let y = {
        let _x = 3;
        _x + 1       // No semicolon here because it's an expression, not a statement
    };

    println!("The value of y is: {}", y);

    let x = five();

    println!("The value of x is: {}", x);

    let z = plus_one(5);

    println!("The value of z is: {}", z);
}

// Rust uses 'snake case' as the conventional style for function and variable names
// 'fn' is the keyword used to declare a function and {} declare the function body
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// Return values have their type declared after the '->' operator
// In Rust, the return value of the function is synonymous with the
// value of the final expression in the block of the body of a function
// This can be thought of as an implicit return. Explicit returns using
// the 'return' keyword are also allowed in Rust but less common
fn five() -> i32 {
    5     // @note: no semicolon because it's an expression whose value we want to return
}

// @note: If there was a semicolon at the end of Line 48, it would be a compiler error
fn plus_one(x: i32) -> i32 {
    x + 1      // Implicitely return 'x + 1'
    //x + 1;   // Compiler error
}
