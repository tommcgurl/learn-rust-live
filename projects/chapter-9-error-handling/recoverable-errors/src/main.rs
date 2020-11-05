use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // expect_in_action();
    // unwrap_in_action();
    let f: Result<std::fs::File, std::io::Error> = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            other_error => panic!("Problem opening the file: {:?}", other_error)
        }
    };

    // Cleaner version using closures
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

}


fn unwrap_in_action() {
    let f = File::open("hello.txt").unwrap();
}

fn expect_in_action() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt. WHAT UP TWITCH or youtube!");
}