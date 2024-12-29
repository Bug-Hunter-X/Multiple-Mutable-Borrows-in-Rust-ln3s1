# Multiple Mutable Borrows in Rust

This example demonstrates the compile-time error that arises when attempting to create multiple mutable references to the same variable in Rust. This is a core aspect of Rust's ownership and borrowing system, designed to prevent data races and ensure memory safety.

## The Problem

Rust's borrow checker disallows multiple mutable borrows of the same data.  The code in `bug.rs` demonstrates this by attempting to create two mutable references (`y` and `z`) to the variable `x`. The compiler will reject this, helping you avoid potential concurrency issues.