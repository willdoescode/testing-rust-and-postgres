#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod models;
pub mod schema;

use rocket::request::{Form};
use rocket::response::{content};
use diesel::prelude::*;
use diesel::{PgConnection};
use dotenv::dotenv;
use std::env;
use models::{Person, NewPerson};
use schema::person::dsl::*;
use schema::person;

#[derive(FromForm)]
struct Human {
	#[form(field = "name")]
	n: String,
	#[form(field = "age")]
	a: i32,
	#[form(field = "gender")]
	g: String
}

#[post("/create_person", data = "<human>")]
fn create_person_endpoint(human: Form<Human>) {
	let conn = connect_client();
	create_person(&conn, &human.n, human.a, &human.g);
}

fn create_person<'a>(conn: &PgConnection, other_name: &'a str, other_age: i32, other_gender: &'a str) -> Person {
	let new_person = NewPerson {
		name: other_name,
		age: other_age,
		gender: other_gender
	};

	diesel::insert_into(person::table)
		.values(&new_person)
		.get_result(conn)
		.expect("Error saving person")
}

fn delete_person(pattern: String) {
	let conn = connect_client();
	diesel::delete(person.filter(name.like(pattern)))
		.execute(&conn)
		.expect("Error deleting posts");
}

fn main() {
	rocket::ignite().mount("/", routes![create_person_endpoint]).launch();
	let conn = connect_client();
	let res = person.filter(gender.eq("male".to_owned()))
		.limit(5)
		.load::<Person>(&conn)
		.expect("Error loading people");

	for p in res {
		println!("{} is {} years old", p.name, p.age)
	}
}

fn connect_client() -> PgConnection {
	dotenv().ok();
	let db_uri = env::var("DATABASE_URL").expect("set DATABASE_URL in .env");

	PgConnection::establish(&db_uri).expect("must have correct db url")
}

