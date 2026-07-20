#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}
//This will work only for T is string, it won't even work for &str
impl TreasureChest<String> {
    fn clean_treasure (&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }
}

impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

enum  Cheesesteak<T> {
    Plain,
    Topping (T),
}

fn main() {

    println!("{}",identity(5));
    println!("{}",identity::<u32>(5));
    println!("{}",identity::<u8>(5));
    println!("{}",identity::<f32>(5.0));
    println!("{}",identity(6.7));
    println!("{}",identity(true));
    println!("{}",identity("Hello"));
    println!("{}",identity(String::from("Hello")));

    println!("{:#?}",make_tuple(5, "Hello"));
    println!("{:#?}",make_tuple( "Hello", 5));
    println!("{:#?}",make_tuple(true, false));
    println!("{:#?}",make_tuple_example(5, 6));

    let golden_chest = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold",
    };
    println!("{}",golden_chest.capital_captain());
    println!("{:?}", golden_chest);

    let mut silver_chest = TreasureChest {
        captain: String::from("BloosSail"),
        treasure: String::from("   Silver.    "),
    };
    println!("{}",silver_chest.capital_captain());
    silver_chest.clean_treasure();
    println!("{:?}", silver_chest);

    let special_chest = TreasureChest {
        captain: String::from("BootyPlunder"),
        treasure: ["Gold", "Silver", "Platinum"],
    };
    println!("{}",special_chest.capital_captain());
    println!("{}",special_chest.amount_of_treasure());
    println!("{:?}", special_chest);

    let mushroom: Cheesesteak<&str> = Cheesesteak:: Topping ("mushroom");
    let ontons: Cheesesteak<String> = Cheesesteak::Topping ("onions". to_string());
    let topping: String = "bacon". to_string();
    let bacon: Cheesesteak<&String> = Cheesesteak::Topping(&topping);
    let plain: Cheesesteak<String> = Cheesesteak::Plain;

}

fn identity<T> (value: T) -> T { 
    value
}

fn make_tuple_example<T> (first: T, second: T) -> (T, T) {
    (first, second)
}

fn make_tuple<T, U> (first: T, second: U) -> (T, U) {
    (first, second)
}








