fn main() {

    let age = 25;
    println!("Hello, world!");

    {
        let handsome = true;
    } //handsome does not exist here

    let text = String::new();

    let candy = String::from("KitKat");

    let mut name = String::from("Kushal");

    name.push_str("Prakash");

    drop(candy);

    let person = String::from("ABC");
    let genuis = person.clone();

    println!("{person}{genuis}");

    let my_stack_val = 2;
    let my_int_ref = &my_stack_val;
    println!("{my_stack_val}{my_int_ref}");

    let my_heap_val = String::from("Toyota");
    let my_heap_ref = &my_heap_val;
    println!("{} {}", my_heap_val, *my_heap_ref);
    //Age variable exist here
}//Age variable does not exist here
