fn main() {
    let  s = String::from("What's up Twitch!");

    let word = first_word(&s[..]);
    println!("The first word in {} is {}", s, word);

    let my_string_literal = "Hello Chat!";
    let word_two = second_word(&my_string_literal[..]);
    println!("The second word is {}", word_two);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut first_index: usize = 0;
    let mut found_first = false;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && !found_first{
            first_index = i + 1;
            found_first = true;
        } else if item == b' ' && found_first {
            return &s[first_index..i];
        }
    }

    if (found_first) {
        &s[first_index..s.len()]
    } else {
        ""
    }

}


