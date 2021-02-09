use super::schema::person;

#[derive(Queryable)]
pub struct Person {
	pub id: i32,
	pub name: String,
	pub age: i32,
	pub gender: String
}

#[derive(Insertable)]
#[table_name="person"]
pub struct NewPerson<'a> {
	pub name: &'a str,
	pub age: i32,
	pub gender: &'a str
}