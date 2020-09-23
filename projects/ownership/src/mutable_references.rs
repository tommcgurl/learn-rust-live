fn main() {
    let mut s = String::from("Hello");
    change(&mut s);

    println!("The value of s is now: {}", s);
    {
        let r1 = &mut s;
        println!("{}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;
    println!("{}", r2);
}

fn change(some_string: &mut String) {
    some_string.push_str(", Chat!")
}