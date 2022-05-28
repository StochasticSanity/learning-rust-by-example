fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // This yields values from a (inclusive) to b (exclusive) in steps of one.
    // for n in 1..101 {

    // Alternatively, a..=b can be used for a range that is inclusive on both ends. 
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
