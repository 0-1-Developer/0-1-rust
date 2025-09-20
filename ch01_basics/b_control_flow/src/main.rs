use std::collections::HashMap;

fn main() {
    println!("=== Conditional Statements ===");

    let number = 7;
    let is_even = number % 2 == 0;

    if is_even {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }

    // if is an expression in Rust - all branches must return the same type
    let result = if is_even { "even" } else { "odd" };
    println!("Using if as expression: {} is {}", number, result);

    println!("\n=== For Loops ===");

    // Range syntax: start..end (exclusive), start..=end (inclusive)
    println!("Range 0..5:");
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();

    println!("Inclusive range 1..=5:");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();

    println!("\n=== While Loop ===");

    let mut countdown = 3;
    while countdown > 0 {
        println!("{}...", countdown);
        countdown -= 1;
    }
    println!("Liftoff! 🚀");

    println!("\n=== Iterating Collections ===");

    // Arrays: [T; N] - stack allocated, fixed size at compile time
    let numbers = [10, 20, 30, 40, 50];
    print!("Array elements: ");
    // .iter() borrows elements as &T
    for num in numbers.iter() {
        print!("{} ", num);
    }
    println!();

    let text = String::from("Rust");
    print!("String chars: ");
    // .chars() returns Iterator<Item=char>
    for ch in text.chars() {
        print!("{} ", ch);
    }
    println!();

    println!("\n=== HashMap Iteration ===");

    let mut grades = HashMap::new();
    grades.insert("Alice", 95);
    grades.insert("Bob", 87);
    grades.insert("Charlie", 92);

    println!("Student grades:");
    // HashMap iteration order is non-deterministic
    for (name, grade) in grades.iter() {
        println!("  {}: {}", name, grade);
    }

    println!("\n=== Function Call ===");

    let sentence = String::from("Hello world from Rust");
    // clone() needed because get_first_word takes ownership
    let first = get_first_word(sentence.clone());
    // &sentence borrows - more efficient than clone()
    let count = count_words(&sentence);
    println!("First word: '{}'", first);
    println!("Word count: {}", count);

    println!("\n=== Loop with break/continue ===");

    for i in 1..=10 {
        if i % 3 == 0 {
            continue;
        }
        if i > 7 {
            break;
        }
        print!("{} ", i);
    }
    println!("(skipped multiples of 3, stopped at 7)");
}

// String parameter takes ownership (move semantics)
fn get_first_word(sentence: String) -> String {
    let mut word = String::new();

    for ch in sentence.chars() {
        if ch == ' ' {
            break;
        }
        word.push(ch);
    }

    word // implicit return
}

// &str parameter borrows - caller retains ownership
fn count_words(sentence: &str) -> usize {
    sentence.split_whitespace().count()
}
