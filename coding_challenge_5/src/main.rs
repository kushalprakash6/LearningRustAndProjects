fn main() {
    println!("Hello, world!");
    let ret = color_to_number("red");
    println!("{ret}");

    let ret = color_to_number("green");
    println!("{ret}");

    let ret = color_to_number("blue");
    println!("{ret}");

    let ret = color_to_number("white");
    println!("{ret}");

    //Non-recursion factorial 

    let mut num = 10;

    let org_num = num;

    let mut res = 1;

    while num > 1 {
        res = res * num;
        num -= 1;
    }

    println!("Factorial of {org_num} is {res}");

    let res = factorial(10);
    println!("Factorial is {res}");

}

fn color_to_number (color: &str) -> i32 {

    // if color == "red" {
    //     return 1;
    // }
    // else if color == "green" {
    //     return 2;
    // }
    // else if color == "blue" {
    //     return 3;
    // }
    // else {
    //     return 0;
    // }

    let res = match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    };

    return res

}

fn factorial (num: i32) -> i32 {
    if num == 1 {
        return 1;
    }
    else {
        return num * factorial(num -1);
    }
}