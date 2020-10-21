fn main() {
  // Mutable string
  // let mut s = String::new();

  let data = "initial contents";

  let s = data.to_string();

  // The above is equivalent to
  // let s = "initial contents".to_string();

  // Also
  // let s = String::from("initial contents");

  // UTF-8 Encoded
    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);


    let mut s3 = String::from("lu");
    s3.push('l');
    combine_strings_with_plus();
    iterating_over_strings();
}

fn combine_strings_with_plus() {
  let s1 = String::from("Hello, ");
  let s2 = String::from("Twitch!");

  let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");


  let s = format!("{}-{}-{}", s1, s2, s3);

  // is equavalent to:
  // let s = s1 + "-" + &s2 + "-" + &s3;


  println!("s is: {}", s);
  println!("s1 is {}", s1);

}

fn iterating_over_strings() {
  for b in "नमस्ते".bytes() {
    println!("{}", b);
  }
}


