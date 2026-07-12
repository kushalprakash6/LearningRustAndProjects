/*
Copyright 2026
Author : Kushal Prakash
Last updated : 12/07/2026
*/
fn main() {
    println!("Hello, world!\nThis is a sample Rust program.");
    println!("I am a \"software engineer\"");
    let filepath = "C:\\Users\\Kushal\\Documents\\RustProjects\\sample.txt";
    println!("File path: {}", filepath);
    let filepath2 = r"C:\Users\Kushal\Documents\RustProjects\sample.txt";
    println!("File path using raw string: {}", filepath2);

    let value: i32 = -15;
    println!("The absolute value is: {}", value.abs());

    let sample_str: &str = "           Hi!!!                           ";
    println!("{}", sample_str.trim());

    println!("{}", value.pow(2));

    let pi: f64 = 3.1415926535897932384;
    println!("The value of pi is: {}", pi);

    println!("The value of pi rounded to 2 decimal places is: {:.2}", pi);
    println!("{}", pi.round());
    println!("{}", pi.floor());
    println!("{}", pi.ceil());

    println!("The current value of pi is: {pi:.3}");

    let distance = 50;
    let dist_i8: i8 = distance as i8;
    println!("The value of distance as i8 is: {}", dist_i8);

    let dist = 10.76;
    let dist_i32: i32 = dist as i32;
    println!("The value of dist as i32 is: {}", dist_i32);

    let div = 10 / 3;
    println!("The value of div is: {}", div);
    let div = 10.0 / 3.0;
    println!("The value of div is: {}", div);

    let rem = 10 % 3;
    println!("The value of rem is: {}", rem);

    let rem = 10.0 % 3.0;
    println!("The value of rem is: {}", rem);

    let age = 28;
    let is_adult = age >18;
    println!("Is the person an adult? {is_adult}");


    println!("{}", "Coke" == "Pepsi");
    println!("{}", "Coke" != "Pepsi");
    println!("{}", "Coke" == "coke");
    println!("{}", "Coke" == "Coke ");
    println!("{}", 12 == 12.0 as i32);

    let char  = 'A';
    println!("The value of char is: {}", char);
    let emoji = '🤓';
    println!("The value of emoji is: {}", emoji);

    let numbers = [1, 2, 3, 4, 5];
    println!("The value of numbers is: {:?}", numbers);

    let os = ["Macintosh", "Windows", "Linux"];
    println!("The value of os is: {os:?}");

    println!("The value of os is: {os:#?}");

    let currency: [f64; 0] = [];
    println!("The value of currency is: {:?}", currency);

    println!("{}", numbers[2]);


    //Trying tuple
    let e_id = (101, "John Doe", 50000.0);

    let name = e_id.1;
    let salary = e_id.2;
    let id = e_id.0;
    println!("Employee ID: {id}, Name: {name}, Salary: {salary}");

    dbg!(e_id);


    let (x, y, z) = (1, 2, 3);
    println!("The value of x is: {x}, y is: {y}, z is: {z}");

    let (id, name, salary) = e_id;
    println!("Employee ID: {id}, Name: {name}, Salary: {salary}");


    let date = 1..31;   //31 not included
    println!(" {date:?}");

    for i in date {
        println!("{i}");
    }

    let date = 1..=31;  //31 included
    println!(" {date:?}");

    for i in date {
        println!("{i}");
    }

    let ch = 'A'..='k';
    for i in ch {
        println!("{i}");
    }

}