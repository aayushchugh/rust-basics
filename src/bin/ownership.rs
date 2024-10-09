/**
* This file covers basics of ownership in rust
*
* 1. Each value has a variable that's its "owner".
* 2. A value can have only one owner at a time.
* 3. If the owner goes out of scope, the value is cleaned up.
*/
fn main() {
    // ownership transfer
    let s1 = String::from("world");
    let s2 = s1; // ownership of "world" is transfered to s2

    // println!("s1 is: {s1}") we will get compile time error in this line because ownership was
    // transfered to s2

    // cloning values instead of transfering ownership
    let s3 = String::from("Hello");
    let s4 = s3.clone(); // This won't transfer ownership to s4 instead, it will clone the value.

    let s5 = String::from("Hello again");
    {
        let s6 = s5; // ownership is transfered to s6.
    }

    // s6 will now be dropped when the inner scope ends
    // we'll get error if we try to access s6 after inner scope

    // There is an exception for primitive datatypes such as int, float, char, bool
    let x: i32 = 10;
    let y: i32 = x;

    println!("Value of X is: {x}"); // This won't throw any error
}
