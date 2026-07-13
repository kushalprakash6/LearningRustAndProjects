/*
Copyright 2026
Author : Kushal Prakash
Last updated : 12/07/2026
*/
fn main() {

    open_store("Frankfurt");
    bake_pizza(2, "cheese and tomato");
    make_money();  
    open_store("Munich");

    let sq_res = square(2);

    println!("The square of 2 is: {sq_res}");

    let add_res = add(2, 4);

    println!("The addition of 2 and 4 is: {add_res}");

    let unknown = mystery();
    println!("{unknown:?}");

    let multiplier = 3;
    // Trying scope
    let calc = {
        let value = 4 + 5;
        value * multiplier
    };

    println!("The result of the calculation is: {calc}");
    

}


fn open_store (place: &str) {
    println!("Opening store in {place}");
}

fn bake_pizza (quantity: u32, toppings: &str) {
    println!("Baking {quantity} pizzas with {toppings}...");
}

fn make_money () {
    println!("Making money...");
}

fn square (num: i32) -> i32
{
    return num * num;
}

fn add (num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn mystery () { }

