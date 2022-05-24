use std::io;

fn main() {
    println!("This program will print the nth Fibonnaci number!");
    println!("Which Fibonnaci number would you like to see?");

    let mut choice = String::new();

    io::stdin()
    .read_line(&mut choice)
    .expect("Failed to read line");

    let choice: u16 = choice.trim().parse().expect("Please type a number!");

    println!("The position {} Fibonacci number is {}", choice, fib(choice));

}

fn fib(x: u16) -> u16 {

    if x == 1 || x == 2 {
        1
    } else {
        fib(x-2)+fib(x-1)
    }
}