// Includes the standard input-output library.
use std::io;
// Includes the standard comparitor library.
use std::cmp::Ordering;
// Includes the random library
use rand::Rng;

// Our main function - program entry point
fn main() {
    // Prints the title of the program
    println!("Guess the number!");
    // Generate a random number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..101);
    // Print the random number
    println!("The secret number is: {}", secret_number);

    // Loop
    loop {
        // Prompt the user for input
        println!("Please input your guess.");
        // Creates a variable to store user input
        // -- mut means the variables is mutable, it can change
        // -- String::new() creates a new String instance.
        let mut guess = String::new();

        // Call the stdin (standard input) function from the input-output
        // library
        io::stdin()
            // stdin gives us the command line to read from.
            // Call read_line to read input from the command line
            // Parameter is reference & to a mutable string
            .read_line(&mut guess)
            // read_line might fail so we have an except call to send message
            .expect("Failed to read line");

        // Convert String guess to u32 (unsigned 32-bit int)
        // Remove any spaces, newlines, etc. from the input
        // Convert (parse) the string to a number based on type
        // Match (deal with) the result of the parse
        let guess : u32 = match guess.trim().parse() {
            // Handle error from parsing
            // If parse Ok, return num (parsed value)
            Ok(num) => num,
            // If parse fails, restart loop
            Err(_) => continue
        };

        println!("You guessed: {}", guess);

        // Compare guess to secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // Exit loop
                break;
            }
        }
    }
}