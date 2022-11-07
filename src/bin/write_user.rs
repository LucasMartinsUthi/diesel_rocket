use diesel_rocket::*;
use std::io::{stdin};

fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();
    let mut location = String::new();
    let mut title = String::new();

    println!("Name?");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end(); // Remove the trailing newline

    println!("Location?");
    stdin().read_line(&mut location).unwrap();
    let location = location.trim_end(); // Remove the trailing newline

    println!("Title?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end(); // Remove the trailing newline


    let user = create_user(connection, &name, &location, &title);
    println!("\nSaved draft {} with id {}", name, user.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";