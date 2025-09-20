fn main() {
    println!("=== Integer Types ===");

    let x: i32 = 1;
    let y = 1; // Type inference defaults to i32

    let z: u8 = 255; // u8::MAX
    let w: f32 = 2.5;

    println!("i32: x = {}, inferred: y = {}", x, y);
    println!("u8 (max 255): z = {}", z);
    println!("f32: w = {}", w);

    println!("\n=== Mutability & Overflow ===");

    let mut a: i8 = 10; // i8: -128..=127
    println!("Initial a (i8): {}", a);

    a += 100;
    println!("After adding 100: {}", a);

    // Overflow behavior differs: debug = panic, release = wrapping
    // a += 20; // 130 > i8::MAX

    println!("\n=== Boolean Logic ===");

    let is_adult: bool = true;
    let has_license: bool = true;

    if is_adult && has_license {
        println!("Can drive: {} && {} = true", is_adult, has_license);
    } else {
        println!("Cannot drive");
    }

    println!("\n=== Characters & Strings ===");

    let letter: char = 'R';
    let emoji: char = '🦀'; // char is 4 bytes, Unicode scalar value
    println!("char: '{}', emoji: '{}'", letter, emoji);

    // &str: borrowed string slice (fat pointer: ptr + len)
    let string_slice: &str = "Hello, Rust!";

    // String: owned, heap-allocated, Vec<u8> internally
    let owned_string: String = String::from("Dynamic string");

    println!("&str: {}", string_slice);
    println!("String: {}", owned_string);

    // Option<T> for safe indexing - no null pointer exceptions
    let safe_char = owned_string.chars().nth(1000).unwrap_or('?');
    println!("Character at index 1000 (default '?'): {}", safe_char);

    println!("\n=== Type Sizes ===");

    println!("i8: {} bytes", std::mem::size_of::<i8>());
    println!("i32: {} bytes", std::mem::size_of::<i32>());
    println!("f64: {} bytes", std::mem::size_of::<f64>());
    println!("bool: {} byte", std::mem::size_of::<bool>());
    println!("char: {} bytes", std::mem::size_of::<char>());
}
