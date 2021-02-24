fn main() {
    println!("Hello, world!");
    destructring_enums()
}


fn matching_literals() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything")
    }
}

fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ =>  println!("at t he end: x = {:?}, y = {:?}", x, y)
    }
}

fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_ranges_of_values() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else")
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ACSII letter"),
        'k'..='z' => println!("late ACSII letter"),
        _ => println!("something else")
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destructuring_structs() {
    let p = Point { x:0, y: 7};
    // Use the below if we want to change the name
    // of the destructured values.
    // let Point {x: a, y: b} = p;
    let Point {x,y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point {x, y: 0} => println!("On the x axis at {}", x),
        Point {x: 0, y} => println!("On the y axis at {}", y),
        Point {x, y} => println!("On neither axis: ({}, {}) ",x, y),
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
fn destructring_enums() {
    // let msg = Message::ChangeColor(0, 160, 255);
    // let msg = Message::Write(String::from("hello"));
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
    }
}


fn destructuring_structs_and_Tuples() {
     let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

fn ignoring_remaining_parts() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

fn match_guards() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (), 
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}

enum OtherMessage {
    Hello {id: i32},
}
fn at_bindings() {
    let msg = OtherMessage::Hello { id: 5};


    match msg {
        OtherMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        OtherMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        OtherMessage::Hello { id } => println!("Found some other id: {}", id),   
    }
}