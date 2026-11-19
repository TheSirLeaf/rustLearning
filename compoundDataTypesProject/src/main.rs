// String  — [growable, mutable, owned string type]
// &str    — [borrowed, read-only view of string data]
// str     — [raw unsized type; rarely used alone]
// &String — [borrowed reference to an owned String; prefer &str]

fn main() {
    // Arrays: fixed-size, stack-stored collections
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number array: {:?}", numbers);

    // [&str; 3] — array of string literals; &str points to static memory, no heap allocation
    let fruits: [&str; 3] = ["Apple", "Banana", "Mango"];
    println!("Fruits array: {:?}", fruits);
    println!("First fruit: {}", fruits[0]);
    println!("Second fruit: {}", fruits[1]);
    println!("Third fruit: {}", fruits[2]);

    // Tuples: fixed-size, mixed-type collections
    // String here because the tuple owns the data — no lifetime concerns
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human tuple: {:?}", human);

    // "Kratos" is &str (string literal); Rust infers (&str, i32, bool, [i32; 5])
    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    // Slices: borrowed view into contiguous data — no copying, no ownership transfer
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slices: {:?}", number_slices);

    // &[&str] — borrowed slice of string literals; two levels of borrowing
    let animal_slices: &[&str] = &["Lion", "Zebra", "Elephant"];
    println!("Animal Slices: {:?}", animal_slices);

    // &[&String] — rare pattern; each element is a reference to an owned String on the heap
    // Prefer &[&str] instead to avoid unnecessary heap allocations
    let book_slices: &[&String] = &[
        &"Lord of the Rings".to_string(),
        &"Harry Potter".to_string(),
        &"The Art of War".to_string(),
    ];
    println!("Book Slices: {:?}", book_slices);

    // String — owned, heap-allocated, growable; use when you need to build or own text
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone cold says: {}", stone_cold);

    // String vs &str conversion: .as_str() gives a borrowed view of owned String data
    let borrowed_view: &str = stone_cold.as_str();
    println!("Borrowed view of stone_cold: {}", borrowed_view);

    // &String automatically coerces to &str — no explicit conversion needed
    let another_view: &str = &stone_cold;
    println!("Another borrowed view: {}", another_view);

    stone_cold.push_str("Yeah!");

    let mut string: String = String::from("Hello, world!");
    let slice: &str = &string[0..5];
    // string = "I am a string!".to_string(); <= This wouldn't work, because the borrow is used
    // later
    println!("Slice value: {}", slice);
    println!("Slice value: {}", &slice[0..2]); // This makes a &str borrow from "slice" (which is a
                                               // borrow, by itself)

    // You can't mutate or reassign the owner until the borrow is last used
    string = "I am a string!".to_string(); // Since "slice" isn't used anymore, this works
    println!("String: {}", string);
}

// fn print(){
//     println!("Slice: {}", slice); <= Doesn't work, "slice" is out of the scope.
// }
