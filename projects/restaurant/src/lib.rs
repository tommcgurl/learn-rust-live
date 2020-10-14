mod front_of_house;


fn serve_order() {}
mod back_of_house {

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn fall(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("apples"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();
    
//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

// Use with an abs path
// use crate::front_of_house::hosting

// Use with an abs path
// use front_of_house::hosting

// Public use that other external code can take advantage of 
pub use crate::front_of_house::hosting;


pub fn eat_at_restaurant() {

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    let mut meal = back_of_house::Breakfast::fall("Rye");

    meal.toast = String::from("Marble Rye");

    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("figs");

    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    
    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
}

