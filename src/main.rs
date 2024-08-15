use std::io;

fn main() {
    println!("Temperature Converter\n");

    println!("----------------------------------------------------");
    println!("Enter 1 to convert from Celsius to Fahrenheit");
    println!("Enter 2 to convert from Fahrenheit to Celsius");
    println!("----------------------------------------------------");

    let choice: u8 = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please enter a valid number"),
        }
    };
    
    if choice != 1 && choice != 2 {
        println!("Invalid choice");
        return;
    }

    println!("Enter temperature to convert: ");
    let temp: f32 = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please Enter a valid number"),
        }
    };

    if choice == 1 {
        println!("{}°C = {}°F", temp, celsius_to_fahrenheit(temp));
    }
    else if choice == 2 {
        println!("{}°F = {}°C", temp, fahrenheit_to_celsius(temp));
    }

    // let mut bleh = String::new();
    // println!("\n\nPress Enter to exit");
    // io::stdin().read_line(&mut bleh).expect("meh");
}

fn celsius_to_fahrenheit(temp: f32) -> f32 {
    (9.0/5.0) * temp + 32.0
}

fn fahrenheit_to_celsius(temp: f32) -> f32 {
    (temp - 32.0) * 5.0/9.0
}