# Rust Mutable vs. Immutable Reference Error

This repository demonstrates a common error in Rust: attempting to modify a value through an immutable reference. The `bug.rs` file shows the problematic code, while `bugSolution.rs` provides a corrected version that illustrates the correct way to manage mutable and immutable references.

This bug arises from Rust's ownership and borrowing system, which ensures memory safety.  Understanding how mutable and immutable references work is essential to writing safe and efficient Rust code.