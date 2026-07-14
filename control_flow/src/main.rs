/*
Copyright 2026
Author : Kushal Prakash
Last updated : 13/07/2026
*/
fn main() {

    let num = 10;

    if num == 1 {
        println!("Number is 1");

    }
    else if num >1 {
        println!("Number is greater than 1");
    }
    else {
        println!("Number is less than 1");
    }

    even_or_odd(17);
    even_or_odd(100);

    let eval = true;

    match eval {
        true => {
            println!("The value is true");
        },
        false => println!("The value is false"),
        _ => unreachable!(),
    }

    let val = match eval {
        true => 40,
        false => 20,
    };

    println!("val is {val}");

    let var_x = 99;

    match var_x {
        10 => println!("var_x is 10"),
        20 => println!("var_x is 20"),
        _ => println!("var_x is neither 10 nor 20"),
    }

        match var_x {
        x if x == 10 => println!("var_x is {x}"),       // Compiler will use var_x value for x
        x if x == 20 => println!("var_x is {x}"),
        _ => println!("var_x is neither 10 nor 20"),
    }

    match var_x {
        2 | 4 | 6 | 8  => println!("var_x is even"),
        1 | 3 | 5 | 7 | 9 => println!("var_x is odd"),
        _ => println!("var_x is not single digit"),
    }


}

fn even_or_odd(num: i32) {
    let res = if num % 2 == 0 {
        "even"
    }
    else {
        "odd"
    };
    println!("the number is {res}");
}
