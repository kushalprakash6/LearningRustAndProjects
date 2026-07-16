fn main() {

    let mut current_meal = String::new();
    add_flour(&mut current_meal);
    show_my_meal(&current_meal);

    let car = String::from("Red");
    let ref1 = &car;
    let ref2 = &car;
    println!("{ref1}{ref2} {}", &car);

    let registrations = [true, false, true];
    let first = registrations[0];

    println!("{first} and {registrations:?}");

    let languages = [String::from("Rust"), String::from("C++")];
    let first = &languages[0];

    println!("{first} and {languages:?}");

    let languages = (String::from("Rust"), String::from("C++"));
    let first = &languages.0;
    let first_1 = languages.0.clone();

    println!("{first} {first_1} and {languages:?}");

}

fn add_flour(mut meal: &mut String) {
    meal.push_str("Add flour");
}


fn show_my_meal (meal: &String) {
    println!("Meal steps: {meal}");
}

