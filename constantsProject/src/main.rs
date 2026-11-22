fn main() {
    let mut x = 5;
    const Y: i32 = 8;

    // const mut y = 8; // Doesn't work, a 'const' can't be 'mut'.
    // const y = 8; // A constant's type must be defined.
    // const y: i32 = 8; // A constant should have an upper case name (Y in this case).
    println!("The value of the variable x is: {}", x);
    println!("The value of the constant Y is: {}", Y);
    println!("Three hours in seconds is {}", THREE_HOURS_IN_SECONDS);
}

// You can declare constants in any scope, including the module-level scope.
// Module-level scope means outside any function, but still within a module.

const PI: f64 = 3.1415;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;



