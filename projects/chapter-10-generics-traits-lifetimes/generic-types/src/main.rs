fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

struct Complex_Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Complex_Point<T, U> {
    fn mixup<V, W>(self, other: Complex_Point<V, W>) -> Complex_Point<T, W> {
        Complex_Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    &largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);

    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let char_list = vec!['t', 'w', 'i', 't', 'c', 'h'];

    let result = largest(&char_list);

    println!("The largest char is {}", result);
    assert_eq!(*result, 'w');

    let integer = Point { x: 5, y: 1 };
    let _float = Point { x: 1.0, y: 4.0 };

    println!("integer.x = {}", integer.x());
    // x is the T, y is the U
    let p1 = Complex_Point { x: 5, y: 10.4 };
    // x is the V, y is the W
    let p2 = Complex_Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    // x is the T, y is the W
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
