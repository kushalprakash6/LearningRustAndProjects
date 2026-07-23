// mod inventory;
// mod orders;

use fake::{Fake, Faker};
use std::{fmt, 
    io::{self, stdin, stdout} 
};

// use inventory::MANAGER;
// use orders::MANAGER as ORDERS_MANAGER;

use project_structure::inventory;
use project_structure::orders;
use project_structure::MANAGER;
use project_structure::ORDERS_MANAGER;


use crate::inventory::products::ProductCategory;

fn main() {
    println!("Hello, world!");
    println!("The manager of our inventory is {}", MANAGER);
    println!("The manager of our orders is {}", orders::MANAGER);

    println!("Our managers are {} and {}. We have {} square feet of floor space", inventory::MANAGER, ORDERS_MANAGER, inventory::FLOOR_SPACE);

    inventory::talk_to_manager();

    let fav_category = inventory::products::ProductCategory::Hammer;
    println!("My favourite category is {:?}", fav_category);

    let tall_ladder = inventory::products::Item {
        name: String::from("Ladder-o-matic 2000"),
        category: fav_category,
        quantity: 5,
    };

    println!("{tall_ladder:?}");

    let fake_item: inventory::products::Item = Faker.fake();
    println!("{:?}", fake_item);

    let fake_item_1: ProductCategory = Faker.fake();
    println!("{:?}", fake_item_1);

}
