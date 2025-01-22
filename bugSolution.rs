fn main() {
    let mut x = 5;
    { // Create a new scope to limit the lifetime of the mutable reference
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // Modify x through y
    }
    println!("x = {}", x); // Output: x = 10
}
//Alternative solution, create a copy
fn main() {
    let mut x = 5;
    let y = x; //y is a copy of x
    let z = &mut y; // z is a mutable reference to y
    *z = 15;
    println!("x = {}, y = {}", x, y); //Output: x = 5, y = 15
}