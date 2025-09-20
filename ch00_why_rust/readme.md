# Chapter 0: Why Rust?

## What You'll Learn

Understanding why Rust matters in modern software development and how it differs from JavaScript/Node.js.

## Why It Matters

While Node.js is excellent for many applications, Rust fills critical gaps in performance, memory safety, and systems programming.

## Core Ideas

### Type Safety
Node.js doesn't enforce type safety at runtime (though TypeScript helps at compile time). Rust enforces type safety at compile time, catching errors before they can run.

```javascript
// JavaScript - Runtime error
let result = "5" + 5;  // "55" (string concatenation)
let calc = result / 2; // 27.5 (suddenly it's a number!)
```

```rust
// Rust - Compile-time error
let result = "5" + 5;  // ERROR: cannot add integer to string
// Must explicitly convert:
let result = "5".parse::<i32>().unwrap() + 5; // 10
```

### Systems Programming
Rust is a systems language with direct access to system resources - perfect for operating systems, embedded systems, and performance-critical applications.

### Performance
Rust compiles to native machine code, making it significantly faster than JavaScript which runs through an interpreter/JIT compiler.

### Memory Safety
Rust's ownership system (owners, borrowing, and lifetimes) guarantees memory safety without a garbage collector.

### Concurrency
Built-in concurrency primitives that prevent data races at compile time.

## Further Resources

- [Course Link](https://projects.100xdevs.com/tracks/rust-bootcamp/Rust-Bootcamp-3)
