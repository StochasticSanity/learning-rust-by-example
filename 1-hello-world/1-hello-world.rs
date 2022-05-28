// This is the main function
fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
    // Click 'Run' above to see the expected output. 
    // Next, add a new line with a second println! macro so that the output shows:
    //     Hello World!
    //     I'm a Rustacean!
    println!("I'm a Rustacean! {}","🦀")

    // There are various optional patterns this works with. Positional arguments can be used.
    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
}