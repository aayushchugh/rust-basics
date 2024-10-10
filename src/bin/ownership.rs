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

    // ownership in functions
    let vec_1 = vec![1,2,3];
    // takes_ownership(vec_1);
    // println!("vec 1 is: {:?}", vec_1); // This will throw an error because ownership was
    // transfer tto function

    // we can clone value of vec_1 in takes_ownership function to bypass this error
    takes_ownership(vec_1.clone());

    let vec_2 = gives_ownership(); // this will give ownership to vec_2
    println!("Vec 2 is {:?}", vec_2);

    let vec_3 = takes_and_gives_ownership(vec_2); // this will take ownership of vec_2 and give it
    // to vec_3
   //  println!("Vec 2 is {:?}", vec_2); // this will give error because owhership of vec_2 is
    // transfered to function
    println!("Vac 3 is {:?}", vec_3);

    // in rust ownership of primitive datatypes like int, char, bool is not transfered to functions
    let x = 10;
    stack_function(x);
    println!("X in main is {x}");
}

fn takes_ownership(vec: Vec<i32>) {
    println!("Vec is {:?}", vec);
}

fn gives_ownership() -> Vec<i32> {
    vec![4,5,6]
}

fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(10);
    vec
}

fn stack_function(mut var: i32) {
    var = 56;
    println!("In function, var is : {var}");
}
