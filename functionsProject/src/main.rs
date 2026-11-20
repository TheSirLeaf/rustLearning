// ---- Functions ----
// A function / variables should be written in snake_case
// The main function is the entry point
fn main() { // "fn main() {..." works, "fn hello() {..." doesn't
    hello_world();
    tell_height(182);
    human_id("Lorran", 12, 152.7);
    let x = {
        let price = 5;
        let quantity = 12;
        price * quantity // No semicolon — this is the block's return value
        // return price * quantity; would work too, but return is for exiting the enclosing function
        // early
    };
    println!("The value of x is: {}", x);
    let y = add(x, 4);
    println!("The value of y is: {}", y);
    println!("The value of 'add' is: {}", add(2, 10));

    // Calling the BMI function
    println!("The value of bmi is: {:.2}", calculate_bmi(87.9, 1.72));
    // {:.2} formats a float to 2 decimal places. Similarly, {:.4} would show 4, {:.1} would show 1.
}

// Unlike JavaScript, Rust has no hoisting. Functions just need to be in the same scope —
// the order of definition doesn't matter because all names are resolved at compile time.
fn hello_world(){
    println!("Hello, world!");
}

// You can insert input values (parameters)
fn tell_height (height: u32){
    println!("My height is: {} cm.", height);
}

// You can insert more than one value
fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old and my height is {} cm.", name, age, height);
}

// ---- Expressions and Statements ----
// Expression is anything that returns a value
// Statements is anything that doesn't return a value

// Expression examples:
// ----------------------------------------
// 5
// true & false
// add(3, 5)
// if condition {value1} else {value2}
// ({code})
// ----------------------------------------

// Statement examples:
// ----------------------------------------
// let x = 5;
// let mut y = 10;
// println!("Hello!");
// x = 100;
// fn my_function() {}
// ----------------------------------------

// Functions can return values
fn add(a: i32, b: i32) -> i32{
    a + b
}

// Final example
// BMI = weight(kg) / height(m)^2
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64{
    weight_kg / (height_m * height_m)
}