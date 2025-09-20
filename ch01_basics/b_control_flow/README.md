# Control Flow Demo

## What You'll Learn
- Conditional statements (if/else)
- Loop constructs (for, while)
- Iterating over collections
- Function definitions and return values

## Core Concepts

### Control Flow
- `if` expressions don't require parentheses (but they're allowed)
- All branches must return the same type

### Loops
- `for` loops with ranges: `0..5` (exclusive end)
- `while` loops for conditional iteration
- Iterator methods: `.iter()` for arrays, `.chars()` for strings

### Collections
- Arrays: Fixed-size, stack-allocated
- HashMap: Key-value store from std::collections
- Strings: Iterate with `.chars()` method

### Functions
- Parameters require type annotations
- Return type must be explicitly declared
- Last expression without semicolon is the return value

## Running the Example
```bash
cargo run
```

## Exercises
1. Modify `get_first_word` to handle empty strings
2. Add a function to count vowels in a string
3. Create a loop that finds prime numbers up to 100

## Key Takeaways
- Rust's control flow is expression-based
- Iterators provide safe, efficient collection traversal
- Function signatures are explicit and checked at compile time