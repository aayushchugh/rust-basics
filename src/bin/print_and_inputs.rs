
/**
 * This file includes print and input statements and Escape sequences
 *
 */
fn main() {
    print!("This is a print command");
    print!("This will be printed on the same line");

    /* Escape sequences
    \n: Newline character.
    \t: Tab space.
    \r: Carriage Return.
    \": Double quote.
    \\: Backward slash
    */

    println!("\n This will be printed after one line ");
    println!("\t A tab space at the start");
    println!("This will be overwritten \r This text will only appear on the screen");

    println!("Prints double quote \", Prints backslash \\");

    println!("I am doing {2} from {1} years and i {0} it", "like", 20, "programming"); // This will take values from list provided by index

    // We can also pick variables from name instead of index.
    println!(
        "{language} is a system programming language which is cool to {activity} in.",
        activity = "code",
        language = "Rust"
    );

    // Input statements
    let mut n: String = String::new();
    std::io
        ::stdin()
        .read_line(&mut n) // &mut will be covered later
        .expect("failed to read input."); // This message will be printed in case of a error

    let n: f64 = n.trim().parse().expect("Invalid input"); // .trim will remove any extra whitespace . .parse will parse the input into explicitly declared type. Expect will print the message in case of error

    println!("{n}");
}
