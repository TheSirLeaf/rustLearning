// Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
// Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
// Floating point: f32, f64
// char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
// bool either true or false
fn main() {
    let x: i32 = -42; // Signed accepts negative value, two's complement
    let y: u64 = 91840; // Unsigned accepts only non-negative, dedicating all its bits to it
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    let e: i32 = 2147483647; // Max value for an i32 signed integer
    let i: i64 = 9223372036854775807; // Max value for an i64 signed integer
    println!("Maximum Value for i32: {}", e);
    println!("Maximum Value for i64: {}", i);

    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    let is_snowing: bool = true; // 'true' or 'false'
    println!("Value of is_snowing: {}", is_snowing);

    let letter: char = 'a'; // 4 bytes
    println!("Value of letter: {}", letter);
}
