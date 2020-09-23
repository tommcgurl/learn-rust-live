fn main() {
    let reference_to_nothing = dangle();
}

// Not going to work as s falls out of scope and
// &s points to nothing
fn dangle() -> &String {
    let s = String::from("hello twitch!");

    &s
}

// works because we actually return the String
fn not_dangle() -> String {
    let s = String::from("hello twitch!");

    s
}