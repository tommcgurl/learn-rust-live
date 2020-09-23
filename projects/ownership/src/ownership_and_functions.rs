fn main() {
    let s = String::from("hello"); // s comes into scope.

    // s' value moves into the function
    take_ownership(s);
    // and is no longer valid here.


    let x = 5; // x comes into scope.

    //x would move into the function,
    makes_copy(x);
    // but i32 is Copy (meaning has trait Copy), so
    // it's okay to still use x afterward.

    println!("x is now: {}", x);

    // println!("s is now: {}", s);


    let s1 = gives_ownership();
    let s2 = String::from("Hello CHAT!");
    let s3 = takes_and_gives_back(s2);

    println!("s1: {}, s3: {}", s1, s3);

    let(s4, len) = calculate_length_takes_ownership(s3);
    // println!("The length of '{}' is: {}", s4, len);

    let len_of_s4 = caluclate_length_with_reference(&s4);
    println!("The length of '{}' is: {}", s4, len_of_s4);

}


fn take_ownership(some_string: String) {
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

// this function will move its return value into the
// function that calls it
fn gives_ownership() -> String {

    // some_string comes into scope
    let some_string = String::from("hello");

    // some_string is returned and moves out to the 
    // calling function
    some_string
}

// will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string is returned and moves out to the calling function.
    a_string
}

fn calculate_length_takes_ownership(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn caluclate_length_with_reference(s: &String) -> usize {
    s.len()
}