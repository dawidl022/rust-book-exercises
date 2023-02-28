use std::str::Split;

use crate::logic::Command;

pub fn parse_command(command: String) -> Result<Command, ()> {
    let mut tokens = command.trim().split(" ");

    let curr = tokens.next();

    if curr.unwrap().to_ascii_lowercase() == "add" {
        parse_add(tokens)
    } else if curr.unwrap().to_ascii_lowercase() == "show" {
        parse_show(tokens)
    } else {
        Err(())
    }
}

fn parse_add(mut tokens: Split<&str>) -> Result<Command, ()> {
    let name = parse_words_until_token(&mut tokens, "to");
    let department = parse_words_until_end(&mut tokens);

    if name.is_empty() || department.is_empty() {
        Err(())
    } else {
        Ok(Command::AddEmployeeToDepartment(name, department))
    }
}

fn parse_words_until_token(tokens: &mut Split<&str>, end: &str) -> String {
    let mut words: Vec<&str> = Vec::new();

    let mut curr = tokens.next();
    while curr.is_some() && curr.unwrap().to_ascii_lowercase() != end {
        words.push(curr.unwrap());
        curr = tokens.next();
    }

    words.join(" ")
}

fn parse_words_until_end(tokens: &mut Split<&str>) -> String {
    let mut words: Vec<&str> = Vec::new();

    let mut curr = tokens.next();
    while curr.is_some() {
        words.push(curr.unwrap());
        curr = tokens.next();
    }
    words.join("")
}

fn parse_show(mut tokens: Split<&str>) -> Result<Command, ()> {
    tokens.next();
    let curr = tokens.next().ok_or(())?;

    if curr == "by" {
        Ok(Command::ShowAll)
    } else if curr == "in" {
        parse_show_in_department(tokens)
    } else {
        Err(())
    }
}

fn parse_show_in_department(mut tokens: Split<&str>) -> Result<Command, ()> {
    let department = parse_words_until_end(&mut tokens);
    Ok(Command::ShowInDepartment(department))
}
