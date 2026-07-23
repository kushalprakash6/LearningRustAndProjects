pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "John Doe";

pub mod products;

pub fn talk_to_manager () {
    println!("Hey {MANAGER}, how's your coffee?");
}

