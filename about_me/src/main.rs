/*
Copyright 2026
Author: Kushal
Date: 03/07/2026

Steps for compiling:
1. Install rust compiler from the official website, steps for installation are updated there.
2. locate the folder and open terminal
3. In terminal, use command cd <folderpath/foldername> to move to the directory
4. Use cargo build command to compile
Code is not compiled.
Task insructions are given below.

Replace the `println!` macro with `print!`. What happens?
- Replacing would cause all the texts to be printed in 1 single line due to no carriage return.
*/

/*
Create a new `about_me` project with the `cargo new` command.

Using the `println!` macro, output 3 sentences about yourself.
Feel free to invoke the macro multiple times.

From the Terminal, compile the `main.rs` file inside the `src`
folder with the Rust compiler, then manually run the executable.

From the Terminal, compile the project with the Cargo tool, then
manually run the executable.

From the Terminal, compile and run the project with a single
Cargo command.

Check your program for errors with `cargo check`.

Add a comment at the top of your source code explaining how to
compile the program for new Rust developers.

Add some spaces and line breaks to the code so that it is formatted
in an ugly manner. From the Terminal, style the code with the
`cargo fmt` command.

Replace the `println!` macro with `print!`. What happens?
*/

fn main() {
    println!("Hello, I am Kushal!");
    println!("I am a software engineer.");
    println!("My expertise include embedded systems and LLMs.");
    println!("I enjoy my free time by fixing broken engines, taking hikes and playing sports.");
}
