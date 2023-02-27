use std::io;

fn main() {
    println!("Select mode:");
    println!("[1] Fahrenheit to Celsius");
    println!("[2] Celsius to Fahrenheit");

    let mut menu_option = String::new();

    io::stdin()
        .read_line(&mut menu_option)
        .expect("Unable to read from stdin");

    let menu_option = menu_option.trim();

    if menu_option != "1" || menu_option != "2" {
        println!("Not a valid option!")
    }

    println!("Enter temperature to convert:");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Unable to read from stdin");

    let temperature: f64 = temperature
        .trim()
        .parse()
        .expect("Temperature must be a real number!");

    let (result, unit) = match menu_option {
        "1" => (fahrenheit_to_celsius(temperature), 'C'),
        "2" => (celsius_to_fahrenheit(temperature), 'F'),
        _ => panic!("Invalid option should not be allowed"),
    };

    println!("The result is {result}°{unit}.")
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}
