mod db;
mod models;
mod error;
mod utils;
mod middlewares;

use crate::db::{user_data_trait::UserDataTrait, Database};
use crate::models::user::{AddUserRequest, UpdateUrl, User};
use crate::utils::consts::DB_CONNECTION_FAILURE_ERROR_MSG;
use crate::middlewares::cors;

use actix_web::web::Data;
use actix_web::{
    web::Path, middleware::Logger, get, patch, post, delete, web::Json, App, HttpServer
};
use error::DataError;
use uuid;
use validator::Validate;


#[get("/users")]
async fn get_users(db: Data<Database>) -> Result<Json<Vec<User>>, DataError>  {
    let users = Database::get_all_users(&db).await;
    match users {
        Some(found_user) => Ok(Json(found_user)),
        None => Err(DataError::NoDataFound),
    }
}

#[post("/newuser")]
async fn add_user(body: Json<AddUserRequest>, db: Data<Database>) -> Result<Json<User>, DataError> {
	let is_valid = body.validate();
	match is_valid {
		Ok(_) => {
			let username = body.username.clone();

            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_user = Database::add_user(&db, User::new(
                String::from(new_uuid),
                username
            )).await;

            match new_user {
                Some(created) => {
                    Ok(Json(created))
                },
                None => Err(DataError::UserCreationFailure),
            }
		}
		Err(_) => Err(DataError::BadRequest)
	}
}

#[patch("/updateuser/{uuid}")]
async fn update_user(update_user_url: Path<UpdateUrl>, db: Data<Database>) -> Result<Json<User>, DataError> {
	let uuid = update_user_url.into_inner().uuid;
    let update_result = Database::update_user(&db, uuid).await;
    match update_result {
        Some(update_user) => Ok(Json(update_user)),
        None => Err(DataError::NoSuchDataFound),
    }
}

#[delete("/deleteuser/{uuid}")]
async fn delete_user(delete_user_url: Path<UpdateUrl>, db: Data<Database>) -> Result<Json<User>, DataError> {
    let uuid = delete_user_url.into_inner().uuid;
    let delete_result = Database::delete_user(&db, uuid).await;
    match delete_result {
        Some(delete_user) => Ok(Json(delete_user)),
        None => Err(DataError::NoSuchDataFound)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
        .await
        .expect(DB_CONNECTION_FAILURE_ERROR_MSG);
    let db_data = Data::new(db);

    HttpServer::new(move || {
        let cors = cors();
        
        App::new()
            .app_data(db_data.clone())
            .service(get_users)
            .service(add_user)
            .service(update_user)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
