fn main() {
    println!("Hello, world!");
    another_function(117, 343);

    let y = -5;
    println!("The value of y is: {}", y);

    let y_plus_three = add_three(y);
    println!("The value of y_plus_three is: {}", y_plus_three);

    let number = -5;
    let is_positive = if number >= 0 { true } else { false };
    println!("The value of is_positive is: {}", is_positive);
    // count_to_ten()
    // countdown_from(10);
    // countdown_to_five();
    countdown_from_five();
}

fn another_function(x: u32, y: u32) {
    println!("The value of x is: {}.\n the value of y is: {}", x, y);
}

fn expression_example() -> u32 {
    let x = 3;
    x + 1
}

fn add_three(x: i32) -> i32 {
  // The reason we are leaving off a semicolon is so that
  // this code will implicitly return.
  x + 3
}

fn divisible_by(number: u32) {

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn count_to_ten() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is: {}", result);
}

fn countdown_from(x: u32) {
    let mut number = x;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn countdown_to_five() {
    let numbers = [1, 2, 3, 4, 5];
    for number in numbers.iter() {
        println!("{}!", number);
    }
}

fn countdown_from_five() {
    for number in (1..6).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!");
}