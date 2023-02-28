use std::{collections::HashMap, io};

pub mod logic;
pub mod parser;

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        print_menu();
        evaluate_command(&mut employees);
        println!()
    }
}

fn print_menu() {
    println!("Company Employee Management System commands:\n");
    println!("Add {{employee}} to {{department}}");
    println!("Show all in {{department}}");
    println!("Show all by department");
}

fn evaluate_command(employees: &mut HashMap<String, Vec<String>>) {
    let command = read_command();
    let command = parser::parse_command(command);

    match command {
        Ok(command) => {
            command.execute(employees);
            println!("{}", command.success_message())
        }
        Err(_) => print_invalid_command(),
    }
}

fn read_command() -> String {
    let mut command = String::new();

    io::stdin()
        .read_line(&mut command)
        .expect("Unable to read from stdin");

    command
}

fn print_invalid_command() {
    println!("Command not recognised!")
}
