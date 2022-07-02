fn main() {
    let x = 5;
    println!("{}",x);
    let x = "6";

    println!("{}",x);

    const SUB_COUNT :u32 = 100_000;
    
    //Integers
    //Floating point
    //Boolean
    //Charecter
    
    let tup = ("lets get rusty", 100_00);
    let (channel, sub_count) = tup;
    let sub_count = tup.1; // tuples are zero indexed

    let error_codes = [200, 404, 500]; 
    // arrya are fixed len 
    // for "dynamic array" use vector

    let byte = [0; 0];// create array len 8 w/ all values == 0

    my_function(11, 22);
}


fn my_function(x :i32, y :i32) {
    println!("Passed in vars: {}", x);
    println!("Passed in vars: {}", y);
}
