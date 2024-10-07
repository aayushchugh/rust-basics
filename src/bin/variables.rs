/**
 * Variables are declared using `let` keyword.
 * 
 * Types of variables
 * 1. Immutable variables
 * 2. Mutable variables
 * 
 * By default, variables are immutable in Rust. To make a variable mutable, use the `mut` keyword.
 */
fn main() {
  // Immutable variable
  let x = 10;

  // Mutable variable
  let mut y = 20;

  // Re-assigning a value to a mutable variable
  y = 30;

  // Printing the values of x and y
  println!("x = {} and y = {}", x, y);

  // Assigning a type to variable
  let z: i32 = 40; // i32 is a 32-bit signed integer type
  println!("z = {}", z);

  // Various types of integer in Rust are as follows:
  // 1. i8 - 8-bit signed integer
  // 2. i16 - 16-bit signed integer
  // 3. i32 - 32-bit signed integer
  // 4. i64 - 64-bit signed integer
  // 5. i128 - 128-bit signed integer
  // 6. isize - signed integer whose size is same as the pointer size of the system
  // 7. u8 - 8-bit unsigned integer
  // 8. u16 - 16-bit unsigned integer
  // 9. u32 - 32-bit unsigned integer
  // 10. u64 - 64-bit unsigned integer
  // 11. u128 - 128-bit unsigned integer
  // 12. usize - unsigned integer whose size is same as the pointer size of the system

  // Shadowing
  // Shadowing is a feature in Rust that allows you to re-declare a variable with the same name.
  // The new variable shadows the previous variable.
  let var: i32 = 50;
  let var: i32 = var + 60;

  // The value of the variable will be the last value assigned to it.
  println!("var = {}", var);

  // Constants
  // Constants are declared using the `const` keyword.
  // Constants are always immutable.
  const MAX_VALUE: i32 = 100; // Constants are written in uppercase with underscores separating words.
  println!("MAX_VALUE = {}", MAX_VALUE);
}