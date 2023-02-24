use std::vec;

use axum::{response::IntoResponse, Json};
use hyper::StatusCode;

use crate::{domain::{entity::user::User, repository::user_repository::UserRepository}, infrastructure::{user_repository::UserRepositoryForDb, utils::establish_connection}, usecase::user_use_case::UserUseCase};

pub async fn all_users() -> impl IntoResponse {
	let pool = establish_connection();
	let usecase = UserUseCase::new(UserRepositoryForDb::new(pool));
	let users = usecase.all().await.unwrap();
	(StatusCode::OK, Json(users))
}

#[cfg(test)]
mod test {
	use crate::{infrastructure::router::create_app, schema::users};

use super::*;
	use axum::{
		body::Body,
		http::{Request},
	};
	use diesel::RunQueryDsl;
use tower::ServiceExt;

	async fn setup() {
		let connection = establish_connection();
		let user = User {
			id: 1,
			name: "test".to_string(),
			email: "test@example.com".to_string(),
			password: "password".to_string(),
		};
		let _ = diesel::insert_into(users::table)
			.values(&user)
			.execute(&mut connection.get().unwrap());
	}

	async fn teardown() {
		let pool = establish_connection();
		let result = diesel::delete(users::table).execute(&mut pool.get().unwrap());
		assert!(result.is_ok());
	}

	#[tokio::test]
	async fn should_return_users() {

		setup().await;

		let req = Request::builder().uri("/users").body(Body::empty()).unwrap();
		let res = create_app().oneshot(req).await.unwrap();
		let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
		let body: String = String::from_utf8(bytes.to_vec()).unwrap();
		let users: Vec<User> = serde_json::from_str(&body).expect("cannot conver User instance.");

		let result = std::panic::catch_unwind(|| {
			assert_eq!(
				users,
				vec![
					User {
						id: 1,
						name: "test".to_string(),
						email: "test@example.com".to_string(),
						password: "password".to_string(),
					},
				]
			);
		});

		if let Err(err) = result {
        std::panic::resume_unwind(err);
    };

		teardown().await;
	}
}