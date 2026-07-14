/*
Copyright 2026
Author : Kushal Prakash
Last updated : 13/07/2026
*/
fn main() {
    apply_to_jobs(100, "embedded systems engineer");

    let res = is_even(8);
    println!("Is 8 even? {}", res);

    let res = is_even(9);
    println!("Is 9 even? {}", res);

    let res:(bool, bool) = alphabets("aardvark");
    println!("{res:?}");
    let res:(bool, bool) = alphabets("zoology");
    println!("{res:?}");
    let res:(bool, bool) = alphabets("zebra");
    println!("{res:?}");

}

fn apply_to_jobs(number : i32, title : &str) {
    println!("I am applying to {number} {title} jobs");
}

fn is_even(num : i32) -> bool{
    if (num% 2) == 0 {
        return true;
    }
    else {
        return false;
    }
}

// Much simpler way
// fn is_even(num : i32) -> bool{
//     num% 2 == 0 
// }

fn alphabets(text : &str) -> (bool, bool) {
    let mut chk_1 = false;
    let mut chk_2 = false;

    for i in text.chars() {
        if i =='a'
        {
            chk_1 = true;
        }
        if i == 'z'
        {
            chk_2 = true;
        }
    }
    return (chk_1, chk_2);
}

// Much simpler way
// fn alphabets(text : &str) -> (bool, bool) {
//     let a = text.contains('a');
//     let z = text.contains('z');
//     (a,z)
// }

// fn alphabets(text : &str) -> (bool, bool) {
//     (text.contains('a'), text.contains('z'))
// }