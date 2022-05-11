mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
/*
    pub mod serving {

        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }

}*/


/*
fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Mango"),
            }
        }
    }
    fn fix_incorrect_order(){
        super::serve_order();
        //crate::front_of_house::serving::serve_order();
    }
}

//use crate::front_of_house::hosting;

//use self::front_of_house::serving::serve_order as back_serve_order;

// Use 'pub use' in order for external code to use the particular function
pub use self::front_of_house::serving::serve_order as back_serve_order;



pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Wheat");

    meal.toast = String::from("White");
    println!("I would like {} bread please!", meal.toast);

    let mut appetizer = back_of_house::Appetizer::Soup;

    //let backserveorder = back_serve_order();
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    //hosting::add_to_waitlist();
}*/
