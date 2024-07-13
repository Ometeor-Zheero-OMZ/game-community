use crate::models::{User};
use crate::{db::Database};
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::{Error};

#[async_trait]
pub trait UserDataTrait {
    async fn get_all_users(db: &Data<Database>) -> Option<Vec<User>>;
    async fn add_user(db: &Data<Database>, new_user: User) -> Option<User>;
    async fn update_user(db: &Data<Database>, uuid: String) -> Option<User>;
    async fn delete_user(db: &Data<Database>, uuid: String) -> Option<User>;
}

#[async_trait]
impl UserDataTrait for Database {
    async fn get_all_users(db: &Data<Database>) -> Option<Vec<User>> {
        let result = db.client.select("user").await;
        match result {
            Ok(all_users) => Some(all_users),
            Err(_) => None,
        }
    }

    async fn add_user(db: &Data<Database>, new_user: User) -> Option<User> {
        let created_user = db
            .client
            .create(("user", new_user.uuid.clone()))
            .content(new_user)
            .await;
        
        match created_user {
            Ok(created) => created,
            Err(_) => None,
        }
    }

    async fn update_user(db: &Data<Database>, uuid: String) -> Option<User> {
        let find_user: Result<Option<User>, Error> = db.client.select(("user", &uuid)).await;

        match find_user {
            Ok(found) => {
                match found {
                    Some(_found_user) => {
                        let updated__user: Result<Option<User>, Error> = db
                            .client
                            .update(("user", &uuid))
                            .merge(User {
                                uuid,
                                username: String::from("名無し")
                            })
                            .await;
                        match updated__user {
                            Ok(updated) => updated,
                            Err(_) => None,
                        }
                    },
                    None => None,
                }
            },
            Err(_) => None,
        }
    }

    async fn delete_user(db: &Data<Database>, uuid: String) -> Option<User> {
        let find_user:  Result<Option<User>, Error> = db.client.select(("user", &uuid)).await;

        match find_user {
            Ok(found) => {
                match found {
                    Some(_found_user) => {
                        let deleted__user: Result<Option<_>, Error> = db
                            .client
                            .delete(("user", &uuid))
                            .await;
                        match deleted__user {
                            Ok(updated) => updated,
                            Err(_) => None,
                        }
                    },
                    None => None,
                }
            },
            Err(_) => None,
        }
    }
}