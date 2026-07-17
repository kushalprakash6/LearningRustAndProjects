fn main() {

    let mut cereals = [String::from("Cookie Crisp"), String::from("Cinnamon Toast Crunch"), String::from("Frosted Flakes"), String::from("Cocoa Puffs"), String::from("Captain Crunch")];
    
    let first_two = &cereals[..2];

    println!("{first_two:?}");

    let middle_three = &cereals[1..4];

    println!("{middle_three:?}");

    let last_three = &mut cereals[2..];

    println!("{last_three:?}");

    last_three[2] = String::from("Lucky Charms");
    println!("{last_three:?}");

    let cookie_crisp = &cereals[0];

    let cookie = &cookie_crisp[..6];
    println!("{cookie}");

    let coca_puffs = &cereals[3];
    let puffs = &coca_puffs[6..];
    println!("{puffs}");

    println!("{cereals:?}");

}
