    #[derive(Debug)]
    struct GlobalCoffee {
        price: f64,
        name: String,
        is_hot: bool,
    }


fn main() {
    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool,
    }

    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 2.99,
        is_hot: true,
    };


    let mut beverage = Coffee {
        name: String::from("Mocha"),
        price: 2.99,
        is_hot: true,
    };

    beverage.name = String::from("Macchiato");
    beverage.price = 3.99;

    println!("Today morning I had {} and it costed {} and was it hot {}", mocha.name, mocha.price, mocha.is_hot);

    println!("Today morning I had {} and it costed {} and was it hot {}", beverage.name, beverage.price, beverage.is_hot);

    let x_name = String::from("Latte");

    let cf = make_coffee(x_name, 2.5, true);

    let milk = GlobalCoffee {
        name: String::from("Milk"),
        ..cf
    };

    println!("Today morning I had {} and it costed {} and was it hot {}", cf.name, cf.price, cf.is_hot);

    println!("Today morning I had {} and it costed {} and was it hot {}", milk.name, milk.price, milk.is_hot);

    no_transfer_ownership(&cf);

    transfer_ownership(cf);

    println!("{:?}", milk);
    println!("{:#?}", milk);

}

fn make_coffee (name:String, price:f64, is_hot:bool ) -> GlobalCoffee {
    GlobalCoffee { price:price, name:name, is_hot:is_hot }
}

fn transfer_ownership (mut global_coffee: GlobalCoffee) {

    global_coffee.price = 5.99;

    println!("Today morning I had {} and it costed {} and was it hot {}", global_coffee.name, global_coffee.price, global_coffee.is_hot);

}

fn no_transfer_ownership (global_coffee: &GlobalCoffee) {

    println!("Today morning I had {} and it costed {} and was it hot {}", global_coffee.name, global_coffee.price, global_coffee.is_hot);

}