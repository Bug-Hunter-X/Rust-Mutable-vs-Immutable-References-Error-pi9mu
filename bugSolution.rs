fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    { // create a new scope for z
        let z = &x;   // z is an immutable reference to x
        println!("x: {}", *z); // Prints 5
    } // z goes out of scope here
    *y +=1;
    println!("x: {}", x); // Prints 6
}
