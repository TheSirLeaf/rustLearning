// Variables and Mutability

// In Rust, variables are immutable by default.
// Once a value is bound to a variable, you cannot change it.
// Use `mut` to make a variable mutable (reassignable).

fn main() {
    // --- Immutable Variables ---
    // A variable declared with `let` cannot be reassigned.
    let x = 5;
    println!("x is {}", x);
    // x = 10; // This would cause a compile error: re-assignment to immutable variable

    // --- Mutable Variables ---
    // Adding `mut` allows you to reassign the variable.
    let mut y = 5;
    println!("y is {}", y);
    y = 10;
    println!("y is now {}", y);

    // --- Type Inference ---
    // Rust infers the type from the value. No need to specify it explicitly.
    let inferred = 42;        // Rust knows this is i32
    let inferred_float = 3.14; // Rust knows this is f64
    println!("inferred: {}, inferred_float: {}", inferred, inferred_float);

    // You can also specify the type explicitly with a colon.
    let explicit: i32 = 42;
    println!("explicit: {}", explicit);

    // --- Shadowing ---
    // You can reuse a variable name to create a new binding that "shadows" the old one.
    // This is different from mutation — it's a brand-new variable.
    let z = 5;
    let z = z + 1; // New `z` shadows the old one; value is now 6
    let z = z * 2; // New `z` shadows again; value is now 12
    println!("z is {}", z);

    // Shadowing allows you to change the type of the value.
    let spaces = "   ";
    let spaces = spaces.len(); // `spaces` is now a usize, not a &str
    println!("spaces length: {}", spaces);

    // --- Constants ---
    // Constants are always immutable, declared with `const`, and must have a type annotation.
    // They can be declared in any scope, including the global scope. More on constants in the next
    // project.
    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);

    // --- `mut` vs `&mut` ---
    // `mut` makes a variable reassignable — you can replace its entire value.
    //   let mut x = 5;  // x can be reassigned: x = 10;
    //
    // `&mut` creates a mutable reference (borrow) — you can modify the value it points to.
    //   let mut x = 5;
    //   let r = &mut x; // r borrows x mutably
    //   *r += 1;        // x is now 6
    //
    // Key difference:
    //   `mut` on a variable → you can reassign the variable itself.
    //   `&mut` on a reference → you can mutate the data through the reference.
}
