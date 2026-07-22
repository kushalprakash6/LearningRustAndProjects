fn main() {

    let movies = ["Ghostbusters", "Spaceballs", "Honey I shrunk the kids"];

    let pizza_dia: Vec<i32> = Vec::new();
    let pizza_dia= Vec::<i32>::new();
    println!("{pizza_dia:?}");

    let mut pizza_diameter = vec![8,12,16];
    pizza_diameter.push(10);
    pizza_diameter.push(14);
    println!("{pizza_diameter:?}");
    pizza_diameter.insert(0,4);
    println!("{pizza_diameter:?}");

    let last_pizza_dia = pizza_diameter.pop();
    println!("{last_pizza_dia:?}");
    println!("{pizza_diameter:?}");

    pizza_diameter.remove(3);
    println!("{pizza_diameter:?}");

    let val = pizza_diameter[2];
    println!("{val}");
    println!("{pizza_diameter:?}");

    let val_1 = pizza_diameter.get(2);
    println!("{val_1:?}");
    println!("{pizza_diameter:?}");

    let peppperoni = String::from("Peppperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");

    let mut pizza_toppings = vec![peppperoni, mushroom, sausage];
    pizza_toppings[1] = String::from("Olives");
    println!("{pizza_toppings:?}");

    let target_topping = &mut pizza_toppings[2];
    target_topping.push_str(" and meatballs");
    println!("{pizza_toppings:?}");
    let another_topping = &mut pizza_toppings[2];
    another_topping.push_str(".");
    println!("{pizza_toppings:?}");
    

    println!("Hello, world!");
}
