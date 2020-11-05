use std::{collections::HashMap, io};

const VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];

fn main() {
    // println!("Hello, world!");
    // let numbers = vec![1, 1, 1, 4, 5, 2, 4, 4, 5, 1];
    // let odd_length_numbers = vec![2, 2, 4];

    // let average = mean(&numbers);
    // println!("Average is: {}", average);

    // let med = median(&numbers);
    // println!("Median is: {}", med);

    // let odd_length_median = median(&odd_length_numbers);
    // println!("Median for odd length vector is: {}", odd_length_median);
    // // println!("Mean of 3 and 4 is {}", mean(&vec![3, 4]));

    // let my_mode = mode(&numbers);
    // println!("Mode is {}", my_mode);
    // // sum: 30
    // // mean of numbers: 3

    // let original_word = String::from("Twitch");
    // let pig_latin_version = convert_to_pig_latin(&original_word);
    // println!("{} in piglatin is {}", original_word, pig_latin_version);
    directory();
}

fn directory() {
    // Add <person> to <department>

    let mut employee_directory = HashMap::new();

    loop {
        println!("Enter a comman like \"Add <Person> to <Department>\"");
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");

        let command: &str = command.trim();
        println!("You entered {}", command);
        let mut iter = command.split_whitespace();
        let person = match iter.nth(1) {
            Some(p) => p,
            None => {
                println!("1. Please enter a valid command");
                continue;
            }
        };
        // let person = iter.nth(1).expect("Invalid command!");
        let department = match iter.nth(1) {
            Some(d) => d,
            None => {
                println!("2. Please enter a valid command");
                continue;
            }
        };

        let employees = employee_directory.entry(String::from(department)).or_insert(vec![]);
        employees.push(String::from(person));

        println!("Employee directory: {:?}", employee_directory);
        // let department = iter.nth(3).expect("Invalid command!");
    }
}

fn convert_to_pig_latin(word: &str) -> String {
    // The first consonant of each word
    // is moved to the end of the word and
    // “ay” is added, so “first” becomes “irst-fay
    // Words that start with a vowel have “hay”
    // added to the end instead
    // (“apple” becomes “apple-hay”)
    let (first, rest) = word.split_at(1);
    let is_vowel = VOWELS.contains(&first);
    if is_vowel {
        return format!("{}-{}", word, "hay");
    }
    format!("{}-{}ay", rest, first)
}

fn mean(numbers: &[i32]) -> f64 {
    // 1. sum the number in the vector
    // 2. Devide by the length of the vector.
    //   let mut sum = 0.0;
    //   for num in numbers {
    //     sum += *num as f64;
    //   }
    let sum = numbers.iter().fold(0, |acc, curr| acc + curr);
    sum as f64 / numbers.len() as f64
}

fn median(numbers: &[i32]) -> f64 {
    // 1. Sort the vector
    // 2. Return middle number
    // if the vector has an even length we return
    // the mean of the two middle numbers.
    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort();
    let middle = sorted_numbers.len() / 2;
    if sorted_numbers.len() % 2 == 0 {
        return mean(&vec![sorted_numbers[middle], sorted_numbers[middle - 1]]);
    }
    sorted_numbers[middle] as f64
}

fn mode(numbers: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for num in numbers {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    println!("Map of occurences {:?}", map);
    let mut max_value = 0;
    let mut mode = 0;
    for (key, value) in map {
        if value > max_value {
            max_value = value;
            mode = *key;
        }
    }
    mode
}
