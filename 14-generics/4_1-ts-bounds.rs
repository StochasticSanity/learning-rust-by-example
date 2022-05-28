struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}
trait Brown {}

impl Red for Cardinal {}
impl Blue for BlueJay {}
impl Brown for Turkey {}

// These functions are only valid for types which implement these
// traits. The fact that the traits are empty is irrelevant.
fn red<T: Red>(_: &T)   -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }
fn brown<T: Brown>(_: &T) -> &'static str { "not red or blue" }

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey   = Turkey;

    // `red()` won't work on a blue jay nor vice versa
    // because of the bounds.
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    println!("A turkey is {}", brown(&_turkey));
    // ^ TODO: Try uncommenting this line.
}
