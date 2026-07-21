use std::collections::btree_map::Values;

#[derive(Debug, Copy, Clone)]
enum MyOption {
    Some(i32),
    None,
}

impl MyOption {
    fn unwrap (self) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Uh, oh"),
        }
    }
}

fn main() {

    let some_option = MyOption::Some(100);
    println!("{:?}", some_option.unwrap());
    let none_option = MyOption::None;
    println!("{:?}", none_option.unwrap());


    let a  = Option::Some(5);
    let b = Option::Some("Hello");
    let c = Option::<i16>::Some(5);
    //let d = Option::None;
    println!("Hello, world!");

    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass = musical_instruments.get(2);

    let availability = is_item_in_stock(true, false);
    println!("{:?}",availability);
    let availability = is_item_in_stock(true, true);
    println!("{:?}",availability);
    let availability = is_item_in_stock(false, false);
    println!("{:?}",availability);
}

fn is_item_in_stock (item_in_system:bool, item_in_stock:bool) -> Option<bool> {
    if item_in_system && item_in_stock {
        Option::Some(true)
    }
    else if item_in_system {
        Option::Some(false)
    }
    else {
        Option::None
    }
}
