use std::io;

fn main() {
    println!("Welcome to the temperature converter v0.1");
        
    loop {
        let mut choice = String::new();
        let mut temp_in = String::new();

        println!("What would you like to do?");
        println!("1: Celsius to Fahrenheit");
        println!("2: Fahrenheit to Celsius");
        println!("3: Quit the program");

        io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

        let choice: u8 = choice.trim().parse().expect("Please type a number!");

        if choice == 1 {
            println!("Enter the temp in degrees:");

            io::stdin()
            .read_line(&mut temp_in)
            .expect("Failed to read line");

            let temp_in: f64 = temp_in.trim().parse().expect("Please type a number!");

            let factor: f64 = 1.8;
            let adjustment: f64 = 32.0;
            println!("{} degrees Celsius = {} degrees Fahrenheit", temp_in, (temp_in*factor)+adjustment);
            continue;
        } else if choice == 2 {
            println!("Enter the temp in degrees:");

            io::stdin()
            .read_line(&mut temp_in)
            .expect("Failed to read line");

            let temp_in: f64 = temp_in.trim().parse().expect("Please type a number!");

            let factor: f64 = 0.55556;
            let adjustment: f64 = 32.0; 

            println!("{} degrees Fahrenheit = {} degrees Celcius", temp_in, (temp_in - adjustment)*factor);
            continue;
        } else { break; }
    }

    println!("Thank you for converting with us");
}
