// Build with:
// rustc 2-using-library.rs --extern rary=lib1_creating_library.rlib

fn main() {
    creating_library::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    creating_library::indirect_access();
}