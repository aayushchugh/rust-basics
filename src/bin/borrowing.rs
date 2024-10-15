/**
 * This file covers basics of borrowing in rust
 *
 * Borrowing establish a reference to some data, it is just like pointers with some rules.
 * Borrowing does not takes ownership of the data.
 *
 * Why borrowing?
 * 1. To avoid ownership transfer
 * 2. To avoid cloning values
 *
 * This will prevent unnecessary memory usage and improve performance.
 *
 * Borrowing rules:
 * 1. At any given time, you can have either one mutable reference or multiple immutable references but not both at the same time.
 * 2. References must always be valid.
 *
 * Problems solved by borrowing
 * 1. Data race
 * 2. Dangling references
 */
fn main() {
    let mut vec_1 = vec![1, 2, 3, 4, 5];
    let ref1 = &vec_1; // Immutable reference
    let ref2 = &vec_1; // Immutable reference

    // In rust, we can't have multiple mutable references to the same data.
    // let mut_ref2 = &mut vec_1; // Error: cannot borrow `vec_1` as mutable more than once at a time.

    println!("ref1: {:?}", ref1);
    println!("ref2: {:?}", ref2);

    let mut_ref1 = &mut vec_1; // Mutable reference

    println!("mut_ref1: {:?}", mut_ref1);

    let vec_2 = {
        let vec_3 = vec![6, 7, 8, 9, 10];
        // &vec_3 // Error: `vec_3` does not live long enough, because it's scope is limited to this block and reference to vec_3 will be dropped when the block ends.
    };

    // takes_ownership(vec_1); // Ownership of vec_1 is transferred to takes_ownership function
    // println!("vec_1: {:?}", vec_1); // Error: value borrowed here after move

    borrow_vec(&vec_1); // Ownership is not transferred, only reference is passed
}

fn borrow_vec(vec: &Vec<i32>) {
    println!("Vec is {:?}", vec);
}
