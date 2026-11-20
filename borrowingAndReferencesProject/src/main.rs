// You can have either one mutable reference OR any number of immutable references to a value at a time.

// Example: One mutable reference at a time — commented out to demonstrate the rule
/*
 fn main(){
    let mut _x: i32 = 5;
    let _r: &mut i32 = &mut _x;
    *_r += 1;
    println!("Value of _x is {}", _x);
    // println!("Value of _r is {}", _r);
} */

fn main(){
    let mut account: BankAccount = BankAccount{
        owner: "Alice".to_string(),
        balance: 150.55
    };
    // Immutable borrow to check the balance    
    account.check_balance();
    // Mutable borrow to withdraw money
    account.withdraw(48.5);
    // Immutable borrow to check the balance
    account.check_balance();
    // These borrows don't overlap because each method call completes before the next one begins.
    // Rust's Non-Lexical Lifetimes (NLL) track the last use of a borrow, not the end of the scope.
}

// A `struct` defines a custom data type by grouping named fields together.
// Think of it like a blueprint: it describes what data a value contains.
struct BankAccount {
    owner: String,  // Field: the account owner's name
    balance: f64    // Field: the current balance
}

// `impl` (implementation) attaches methods to a struct.
// It's how you define what actions a type can perform.
impl BankAccount {
    // `&mut self` — this method mutates the struct (changes the balance).
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from the account owned by {}", amount, self.owner);
        self.balance -= amount;
    }
    // `&self` — this method only reads the struct, no mutation needed.
    fn check_balance(&self){
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}