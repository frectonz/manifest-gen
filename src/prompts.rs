use inquire::{Select, Text};
use std::fmt::Display;

pub fn ask_for_string(question: &str, default: &str) -> String {
    Text::new(question)
        .with_default(default)
        .prompt()
        .unwrap_or_else(|_| String::from(default))
}

pub fn ask_for_select<T: Display>(question: &str, options: Vec<T>, default: T) -> T {
    Select::new(question, options).prompt().unwrap_or(default)
}
