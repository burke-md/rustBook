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
    let s3 = s2.clone();
}
