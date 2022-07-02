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

    let sum = my_function(11, 22);
    println!("The sum is: {}", sum);
    
    loops();
}


fn my_function(x :i32, y :i32) -> i32 {
    println!("Passed in vars: {}", x);
    println!("Passed in vars: {}", y);
    x + y
}


fn loops() {
    let mut counter = 0;

    let result = loop {
         counter += 1;
         if counter == 10 {
         break counter; //returns counter
         }
    };

    println!("The result is {}", result);

    //While loop 
    let mut num = 3;
    while num != 0 {
        num -= 1;
    }

    //For in loop
    let a = [1, 2, 3];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //For in using range 
    for number in 1..4 { // last param in range is exclusive -> 1 2 3
        println!("the value is: {}", number);
    }

}
