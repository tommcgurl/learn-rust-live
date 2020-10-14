struct User {
  username: &str,
  email: &str,
  sign_in_count: u64,
  active: bool,
}


fn main() {

  let mut user1 = User {
    email: "younggods117@twitch.tv",
    username: "yovnggod",
    active: true,
    sign_in_count: 1,
  };

  println!("User's email is {}", user1.email);

  user1.username = String::from("TomMcGurl");

  println!("User's name is {}", user1.username);


  let user2 = build_user(String::from("test@testington.com"), String::from("testTestingsworth"));

  println!("User2 email: {}, username: {}", user2.email, user2.username);


  let inactive_user2 = User {
    active: false,
    ..user2
  };

  println!("Inactive User2 Info - active: {}, username: {}", inactive_user2.active, inactive_user2.username);

}


fn tuple_structs() {
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);

  let black = Color(0,0,0);
  let origin = Point(0,0,0);

  let (x, y, z ) = origin;
  let (r, g, b ) = black;
}


fn build_user(email: String, username: String) -> User {
    User {
       email,
       username,
       active: true,
       sign_in_count: 1,
    }
  }