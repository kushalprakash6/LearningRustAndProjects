/*
Copyright 2026
Author : Kushal Prakash
Last updated : 11/07/2026
*/

const PI: f64 = 3.14159265358979323846264338327950288;  // Constant variable, type is f64
type Kgs = f64;  // Type alias, kgs is now an alias for f64
fn main() {
    println!("Testing variables");

    let mut apples = 100;
    let oranges = 20+40;
    let fruits = apples + oranges;
    apples = 67;  // Make the variable mutable
    let _name = "Kushal";   // Unused variable, so we prefix it with an underscore to avoid warnings
    println!("My garden has {apples} apples and {oranges} oranges, for a total of {fruits} fruits.");
    println!("My garden has {1} apples and {0} oranges, for a total of {2} fruits.", oranges, apples, fruits);


    println!("Taking input and converting it to integer");

    let weight = "70";
    println!("Weight is {weight} and its type is {}", std::any::type_name::<&str>());

    let weight = 70.00;
    println!("Weight is {weight} and its type is {}", std::any::type_name::<f64>());

    let weight = 70;
    println!("Weight is {weight} and its type is {}", std::any::type_name::<i32>());


    println!("Testing scope of variables");
    {
        println!("Inside the scope");
        let weight = 80;
        println!("weight is {weight}");
        let number = 67;

        println!("number is {number}");
        println!("end of the scope");
    }

    println!("number is not accessible outside the scope");  // This will throw an error as number is not accessible outside the scope
    println!("weight outside the scope is {weight}");  // This will print the weight variable defined outside the scope


    println!("Testing constants");
    println!("The value of PI is {PI} and its type is {}", std::any::type_name::<f64>());

    println!("Testing type alias");

    let weight: Kgs = 70.00;  // Using the type alias Kgs

    println!("Weight is {weight} and its type is {}", std::any::type_name::<Kgs>());

    #[allow(unused_variables)]  // This is an attribute to allow unused variables in the scope of the main function
    let unused_var = 0;  // This variable is unused, but we have allowed it to avoid warnings 
}
