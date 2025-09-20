# Types Demo

## What You'll Learn
- Integer types (signed: i8-i128, unsigned: u8-u128)
- Floating point types (f32, f64)
- Boolean and character types
- String types (&str vs String)
- Variable mutability

## Core Concepts

### Integer Types
- **Signed**: i8, i16, i32, i64, i128 (can be negative)
- **Unsigned**: u8, u16, u32, u64, u128 (only positive)
- Default integer type is i32
- Use the smallest type that fits your data range

### String Types
- `&str`: String slice, borrowed reference, usually static
- `String`: Heap-allocated, growable, owned string

### Mutability
Variables are immutable by default. Use `mut` keyword for mutable variables.

## Running the Example
```bash
cargo run
```

## Key Takeaways
- Rust's type system prevents common errors at compile time
- Integer overflow is caught in debug mode
- Option types handle potential absence of values safely
- Explicit mutability makes code intentions clear