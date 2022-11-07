use self::models::*;
use diesel::prelude::*;
use diesel_rocket::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .limit(5)
        .load::<User>(connection)
        .expect("Error loading users");

    println!("Displaying {} Users", results.len());
    for user in results {
        println!("{}", user.name);
        println!("{}", user.location);
        println!("{}", user.title);
        println!("-----------\n");
    }
}