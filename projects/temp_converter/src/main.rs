use std::io;

// fn main() {
//     println!("Please input a temperature in Fahrenheit.");

//     let mut fahrenheit = String::new();
//     // this is a comment

//     io::stdin()
//         .read_line(&mut fahrenheit)
//         .expect("Failed to read line");

//     let fahrenheit: f64 = match fahrenheit.trim().parse() {
//         Ok(num) => num,
//         Err(_) => {
//             panic!("Please input a number.");
//         }
//     };

//     let celsius = fahrenheit_to_celsius(fahrenheit);
//     println!("{}F is equal to {}C", fahrenheit, celsius);
// }

fn main() {
    println!("Please input a temperature...");

    let mut temperature = String::new();
    // this is a comment

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let (temperature, unit) = get_temperature_from_string(temperature.trim());

    if unit == "celsius" {
        println!(
            "{}C is equal to {}F",
            temperature,
            celsius_to_fahrenheit(temperature)
        );
    } else if unit == "fahrenheit" {
        println!(
            "{}F is equal to {}C",
            temperature,
            fahrenheit_to_celsius(temperature)
        );
    }

    // let fahrenheit = celsius_to_fahrenheit(celsius);
    // println!("{}C is equal to {}F", celsius, fahrenheit);
}

fn get_temperature_from_string(temp_string: &str) -> (f64, &str) {
    let temp_string_bytes = temp_string.as_bytes();
    let mut temperature: &str = "";
    let mut unit = "fahrenheit";
    for (i, &item) in temp_string_bytes.iter().enumerate() {
        if item == b'F' || item == b'f' {
            temperature = &temp_string[0..i];
            break;
        } else if item == b'C' || item == b'c' {
            temperature = &temp_string[0..i];
            unit = "celsius";
            break;
        }
    }
    let temperature: f64 = match temperature.parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid temperature!"),
    };
    return (temperature, unit);
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0) / 5.0 + 32.0
}
