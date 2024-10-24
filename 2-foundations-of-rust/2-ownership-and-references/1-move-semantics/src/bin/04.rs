//! Refactor this code so that instead of passing `s` into the `create_string` function.
//! The String gets created in the function itself and passed back to the main
//! function.

fn main() {
    // Create s1 using the create_string function
    let mut s1 = create_string();

    println!("{} == `{}`", stringify!(s1), s1);

    s1.push_str(" World!");

    println!("{} == `{}`", stringify!(s1), s1);
}

/// `create_string()` creates and returns a new `String`
fn create_string() -> String {
    let s = String::from("Hello");

    s
}
