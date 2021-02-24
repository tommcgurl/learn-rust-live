fn main() {
    let favorite_color: Option<&str> = None;

    let is_tuesday = true;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is stream day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as  the background color")
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index)
    }

    let (x, y, z) = (1,2,3);

    let (z, w, _) = (4,5,6);

    let point = (3,5);
    print_coordinates(&point);

}


fn foo(x: i32) {

}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({},{})", x, y)
}