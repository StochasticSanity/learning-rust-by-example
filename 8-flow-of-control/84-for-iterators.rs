
// iter - This borrows each element of the collection through each iteration. 
// Thus leaving the collection untouched and available for reuse after the loop.
fn main_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];
    println!("= Iter =");

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);
}

// into_iter - This consumes the collection so that on each iteration the 
// exact data is provided. Once the collection has been consumed it is no 
// longer available for reuse as it has been 'moved' within the loop.
fn main_into_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];
    println!("= Into Iter =");

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
    //println!("names: {:?}", names);
    // FIXME ^ Comment out this line
}

// iter_mut - This mutably borrows each element of the collection, 
// allowing for the collection to be modified in place.
fn main_iter_mut() {
    let mut names = vec!["Bob", "Frank", "Ferris"];
    println!("= Iter Mut =");

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}

fn main() {
    main_iter();
    main_into_iter();
    main_iter_mut();
}
