use std::collections::HashMap;

fn main() {
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Red"), 50);

  let team_name = String::from("Blue");
  let score = scores.get(&team_name);

  println!("{}'s score is: {:?}", team_name, score);

  for (key, value) in &scores {
    println!("{}'s score is: {}", key, value)
  }

  overwriting_a_value();
  only_insert_value_if_key_has_no_value();
  update_value_based_on_old_value();
}

fn vector_of_tuples_to_hashmap() {
  let teams = vec![String::from("Blue"), String::from("Red")];
  let initial_scores = vec![10, 50];

  // We can use <_,_> becuase Rust will infer the type as <String, i32>
  let mut scores: HashMap<_,_> =
    teams.into_iter().zip(initial_scores.into_iter()).collect();
}

fn ownership_and_hashmaps() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("{}", field_name);
}

fn overwriting_a_value() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}

fn only_insert_value_if_key_has_no_value() {
    let mut scores = HashMap::new();
    // Initial value
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn update_value_based_on_old_value() {
    let text = "hello twitch wonderful twitch";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
