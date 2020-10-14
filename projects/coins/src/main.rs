#[derive(Debug)]
enum UsState {
  NewYork,
  NewJersey,
  Florida,
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}


fn main() {
  let penny = Coin::Penny;


  let jersey = Coin::Quarter(UsState::NewJersey);
  let result = value_in_cents(jersey);
  println!("The result was {}", result);
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => {
      println!("Lucky penny!");
      1
    }
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    }
  }
}
