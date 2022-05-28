// Would you like a challenge? Fix the following example to use if let:

// This enum purposely neither implements nor derives PartialEq.
// That is why comparing Foo::Bar == a fails below.
// enum Foo {Bar}

// fn main() {
//     let a = Foo::Bar;

//     // Variable a matches Foo::Bar
//     if Foo::Bar == a {
//     // ^-- this causes a compile-time error. Use `if let` instead.
//         println!("a is foobar");
//     }
// }

// This enum purposely neither implements nor derives PartialEq.
// That is why comparing Foo::Bar == a fails below.
enum Foo {Bar}

fn main() {
    let a = Foo::Bar;

    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
    // ^-- this causes a compile-time error. Use `if let` instead.
        println!("a is foobar");
    }
}
