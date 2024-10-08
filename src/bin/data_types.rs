use std::{collections::hash_set, hint, vec};

/**
 * This file covers the basic data types used in Rust.
 *
 * Rust compiler can infer the type of a variable based on the value assigned to it.
 * If the compiler is unable to infer the type, we can explicitly specify the type of the variable.
 *
 * There are two types of data types in Rust:
 * 1. Scalar data types
 * 2. Compound data types
 *
 * Scalar data types:
 * 1. Integer
 * 2. Floating-point
 * 3. Boolean
 * 4. Character
 *
 * Type aliases:
 * Type aliases are used to give a new name to an existing type.
 *
 * Type conversion:
 * We can convert one type to another using the `as` keyword.
 *
 * Compound data types:
 * 1. &str and String
 * 2. Arrays
 * 3. Vectors
 * 4. Tuples
 * 5. Empty Tuple
 */
fn main() {
  /* ---------------------------- Scalar data types --------------------------- */

  // Integer data type
  let x: i32 = 10; // i32 is a 32-bit signed integer type
  let y: u32 = 20; // u32 is a 32-bit unsigned integer type

  // Floating-point data type
  let z: f32 = 30.5; // f32 is a 32-bit floating-point type
  let w: f64 = 40.5; // f64 is a 64-bit floating-point type

  // Platform specific integers
  let a: isize = 50; // isize is a signed integer whose size is same as the pointer size of the system
  let b: usize = 60; // usize is an unsigned integer whose size is same as the pointer size of the system

  let is_rust_fun: bool = true; // Boolean data type

  let character: char = 'A'; // Character data type

  type Age = u32; // Type alias for u32
  let age: Age = 25;

  let age2 = age as u64; // Type conversion


  /* ------------------------- Compound data types --------------------------- */

  // &str and String
  let fixed_str: &str = "Hello, World!"; // &str is a string slice which has fixed length and are immutable.
  let mut flexible_str: String = String::from("This string can grow"); // String is available in the standard library and is a growable string.
  // :: is used to access associated functions of a type or a module
  // String type is mutable and growable, we can add more characters to it.
  flexible_str.push('2'); // Adding a character to the string using `.push`

  // Arrays
  let arr: [i32; 5] = [1, 2, 3, 4, 5]; // Array of 5 elements of type i32
  let num = arr[2]; // Accessing the third element of the array

  println!("{:?}", arr); // Printing the entire array
  // {:?} syntax is used to print the entire array or vector

  let arr_with_default_values: [i32; 3] = [0; 3]; // Array of 3 elements with default value 0
  println!("{:?}", arr_with_default_values);

  // Arrays are fixed-size and elements are stored in contiguous memory locations.

  // Vectors
  let vec_1: Vec<i32> = vec![1, 2, 3, 4, 5]; // Vector of 5 elements
  // Vec is a growable array type in Rust. There is no size information in vectors.
  // Just like array, all elements in a vector must be of the same type.
  let num = vec_1[2]; // Accessing the third element of the vector

  // Tuples
  // Tuples are contrast of array that can store multiple values of different types.
  let my_info: (&str, i32, &str, i32) = ("Salary", 400000, "Age", 40); // Tuple with 4 elements
  let salary_value = my_info.1; // Accessing the second element of the tuple

  // Destructuring an entire tuple
  let (salary_label, salary_value, age_label, age_value) = my_info;
  let unit: () = (); // Empty tuple, also known as unit type

}
