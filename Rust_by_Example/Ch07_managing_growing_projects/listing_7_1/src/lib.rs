#![allow(unused)]

mod front_of_house;


mod back_of_house {
	pub struct Breakfast {
		pub toast: String,
		seasonal_fruit: String,
	}

	impl Breakfast {
		pub fn summer(toast: &str) -> Breakfast {
			Breakfast {
				toast: String::from("toast"),
				seasonal_fruit: String::from("peaches"),

			}
		}
	}

	pub enum Appetizer {
		Soup,
		Salad,
	}

	fn fix_incorrect_order() {
		cook_order();
		super::deliver_order();
	}
	
	fn cook_order() {}
}

use crate::front_of_house::hosting;

fn deliver_order() {}

mod customer {
	use super::back_of_house::Appetizer;


	pub fn eat_at_restaurant(){
		// Absolute path
		//crate::front_of_house::hosting::add_to_waitlist();

		// adding use above changes the path we need to transverse
		super::hosting::add_to_waitlist();

		// Relative Path
		super::front_of_house::hosting::add_to_waitlist();


		// Order a breakfast in the summer with Rye toast
		let mut meal = super::back_of_house::Breakfast::summer("Rye");
		// Change our mind about what kind of bread we'd like.
		meal.toast = String::from("Wheat");
		println!("I'd like {} toast please.", meal.toast);

		// The next line will not compile if we uncomment it; we'er not
		// allowed to see or modify the seasonal fruit that comes
		// with the meal
		//meal.seasonal_fruit = String::from("blueberries");

		let order1 = Appetizer::Soup;
		let orded2 = super::back_of_house::Appetizer::Salad;

	}
}