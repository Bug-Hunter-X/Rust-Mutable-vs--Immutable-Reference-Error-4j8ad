fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &y; // z is an immutable reference to y
    *y = 10; // Modify x through y
    println!("x = {}", x); // Output: x = 10
    // The following line is problematic:
    // *z = 15; // This would result in a compile-time error because z is an immutable reference
}