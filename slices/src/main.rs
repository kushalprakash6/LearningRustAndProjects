fn main() {

    let action_hero = String::from("John Wick");
    let string_ref = &action_hero;
    println!("{string_ref}");

    let first_name = &action_hero[0..4];
    println!("{first_name}");

    let last_name = &action_hero[5..9];
    println!("{last_name}");

    do_hero_stuff(&action_hero);

    let another_hero_name = "Doctor Strange";
    do_hero_stuff(another_hero_name);

    let food = "pizza";
    println!("{}", food.len());

    let pizza_slice = &food[0..3];
    println!("{pizza_slice} {}", pizza_slice.len());

    let emoji = "🥺";
    println!("{emoji} {}", emoji.len());

    let values = [2,45,56,656,654,64];

    let my_slice = &values[..3];

    println!("{my_slice:?}");

    let mut my_arr = [12,354,56,456,564];
    let my_slice =  &mut my_arr[2..4];

    println!("{my_slice:?}");

    my_slice[0]= 100;

    println!("{my_slice:?}");
    println!("{my_arr:?}");


}

fn do_hero_stuff (hero_name: &str) {
    println!("{hero_name} saves the day!!!");
}
