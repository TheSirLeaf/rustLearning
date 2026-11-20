// Ownership, Borrowing and References

// ---- Ownership ----
// C, C++ require manual memory management — error-prone and tedious.
// Garbage-collected languages automate it, but add runtime overhead.
// Rust's ownership system provides memory safety without a garbage collector.
// ---- What is Ownership? ----
// Every value has an owner
// There can only be a single owner at a time
// When the owner goes out of scope, the value will be dropped

// ---- 1 ----
// Every value has a owner

/*
fn main(){
    let s1 = String::from("RUST"); // s1 owns the String "RUST"
    let len = calculate_length(&s1); // And as such, calculate_length() can borrow from it
    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize{
    s.len()
} */

// ---- 2 ----
// There can only be a single owner at a time

/*
fn main(){
    let s1 = String::from("RUST");
    let s2 = s1;
    println!("The value of s1 is: {}", s1); // Will fail, s1 isn't the owner of the value anymore
    println!("The value of s1 is: {}", s2); // Works, s2 is the new owner
} */

// ---- 3 ----
// When the owner goes out of scope, the value will be dropped

fn main(){
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
}

/*
fn print_lost(s: &String){
    println!("The value of s is: {}", &s1); // Causes an error, s1 is not defined in this fn scope
} */

fn calculate_length(s: &String) -> usize {
    s.len()
}