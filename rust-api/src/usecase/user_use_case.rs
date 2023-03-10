use anyhow::Ok;

use crate::{domain::{repository::user_repository::UserRepository, entity::user::User}, infrastructure::models::user::{NewUser, UpdateUser}};
use crate::presentation::controller::user_controller::CreateUser;

pub struct UserUseCase<T: UserRepository> {
		user_repository: T,
}

impl<T: UserRepository> UserUseCase<T> {
		pub fn new(user_repository: T) -> Self {
				UserUseCase { user_repository }
		}

		pub async fn all(&self) -> anyhow::Result<Vec<User>> {
				self.user_repository.all()
		}

		pub async fn create(&self, payload: CreateUser) -> anyhow::Result<User> {

			let new_user = NewUser{
				name: &payload.name,
				email: &payload.email,
				password: &payload.password,
			};

			let result = self.user_repository.create(new_user);
			Ok(result.unwrap())
		}

		pub async fn update(&self, id: i32, payload: UpdateUser) -> anyhow::Result<User> {
			let result = self.user_repository.update(id, payload);
			Ok(result.unwrap())
		}

		pub async fn delete(&self, id: i32) -> anyhow::Result<User> {
			let result = self.user_repository.delete(id);
			Ok(result.unwrap())
		}
}