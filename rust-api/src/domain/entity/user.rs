use diesel::{Queryable};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Queryable)]
pub struct User {
		pub id: i32,
		pub name: String,
		pub email: String,
		pub password: String,
}