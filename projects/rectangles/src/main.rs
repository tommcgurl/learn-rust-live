#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
  let rect = Rectangle {
      width: 30,
      height: 50,
  };

  println!("rect is {:#?}", rect);

//   println!(
//     "the are of the rectangle is {} square pixels.",
//     area(&rect1)
//   );
  println!(
    "the are of the rectangle is {} square pixels.",
    rect.area()
  );

  let rect1 = Rectangle {
      width: 30,
      height: 50,
  };
  let rect2 = Rectangle {
      width: 10,
      height: 40,
  };
  let rect3 = Rectangle {
      width: 60,
      height: 45,
  };

  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

  let hip_to_be_square = Rectangle::square(42);

  println!("hip_to_be_square: {:#?}", hip_to_be_square);

}

// fn area(rectangle: &Rectangle) -> u32 {
  // let (width, height) = dimensions;
  // width * height
// }
