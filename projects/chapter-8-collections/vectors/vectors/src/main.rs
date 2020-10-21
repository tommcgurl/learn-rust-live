fn main() {
  // Explicitly declairing the type.
  let v: Vec<i32> = Vec::new();

  // Letting Rust infer the type
  let v_inferred = vec![1, 2, 3];

  let third: &i32 = &v_inferred[2];
  println!("The third element is {}", third);

  match v_inferred.get(3) {
    Some(third) => println!("The fourth element is {}", third),
    None => println!("There is no fourth element."),
  }

  let mut v_mutable = Vec::new();

  v_mutable.push(5);
  v_mutable.push(6);
  v_mutable.push(7);
  v_mutable.push(8);

  {
    let scope_v = vec![1, 2, 3, 4, 5];
    // do stuff with v
  } // <- V goes out of scope and is freed here
  loop_over_vector();
  vector_of_enums();
}

fn immutable_borrow_and_mutable_borrow_in_same_scope() {
    let mut v = vec![1,2,3,4,5];

    // Perform an immutable borrow
    let mut first = &v[0];
    // v.(6);

    println!("The first element is: {}", first);
}

fn loop_over_vector() {

    // Immutable vector loop
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // Mutable vector loop
    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50;
        println!("{}", i);
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector_of_enums() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("orange")),
        SpreadsheetCell::Float(13.37)
    ];

    for i in &row {
        println!("{:?}", i);
    }
}