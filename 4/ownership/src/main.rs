fn main() {
    // Ownership rules
    // 1 Each value in Rust has a variable thats called its owner
    // 2 there can only be one owner at a time
    // 3 when the owner goes out of scope the value will be dropped
    

    let s1 = String::from("string");
    // s1 has:
    //      pointer to heap
    //      len
    //      capacity
    
    let s2 = s1;
    // DOES NOT clone.
    // This is called a 'move' (invalidates s1)
   
    // Clone
    let _s3 = s2.clone();

    let x1 = 5;
    let _x2 = x1; // This is a copy NOT a move. 
    // Rust allows the int type to do this because of a specific trait.
    

    let s = String::from("ownership");
    takes_ownership(s);
    
    // println!("{}", s);
    // This generates error because passing a var into a func moves the value
    // and the variable is then dropped.

    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    }
}
