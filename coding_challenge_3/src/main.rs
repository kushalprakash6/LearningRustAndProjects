/*
Copyright 2026
Author : Kushal Prakash
Last updated : 12/07/2026
*/
fn main() {
    let int_var = 1_337;

    let int_var_16 = int_var as i16;

    let flt_var = 67.676754354654;
    println!("{flt_var:.3}");

    let with_milk = true;
    let with_sugar = false;

    let is_my_type_of_coffee = with_milk && with_sugar;

    let is_acceptable_coffee = with_milk || with_sugar;


    let num: [i8; 4] = [1, 2, 3, 4];

    println!("{:?}", num);

    println!("{num:#?}");

    dbg!(num);

    let tup = (6, 6.7, true, num);

    println!("{:?}", tup);

    println!("{tup:#?}");

    dbg!(tup);

}
