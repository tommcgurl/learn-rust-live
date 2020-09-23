fn main() {
    // let mut s = String::from("hello");

    // s.push_str(", Twitch!");
    // println!("{}", s);

    // Integers are simple values and stored on the stack
    // so in this case y is a copy of 5.
    let x = 5;
    let y = x;

    let tup: (u32, bool) = (117, true);
    let tup_clone = tup;
    
    let (tup_x, _) =  tup;
    let (tup_clone_x, _) =  tup_clone;

    println!("x = {}, y = {}", x, y);
    println!("tup_x = {}, tup_clone_x = {}", tup_x, tup_clone_x);

    // This is an example of a "move"
    // let s1 = String::from("hello");
    // // let s2 = s1;

    // println!("s1 is: {}", s1);


    // This is an example of a "clone"
    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 is: {}, s2 is :{}", s1, s2);
}

// this scope is now over, and s is no longer valid

