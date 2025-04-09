use std::sync::Arc;

use async_trait::async_trait;
use rusqlite::Connection;

use crate::{config::db::DbConfig, domain::{user::{Credentials, User}, user_api::UserApi}, error::errors::{QueryUserError, UserUpdateError}};

pub struct UserService {
    db_config: Arc<DbConfig>
}

impl UserService {
    pub fn new(db_config: Arc<DbConfig>) -> Self {
        Self {
            db_config
        }
    }
}

#[async_trait]
impl UserApi for UserService {
    async fn find_by_email(&self, email: &str) -> Result<User, QueryUserError> {
        let db = self.db_config.get_database().to_owned();
        let owned_email = email.to_owned();
        tokio::task::spawn_blocking(move || {
            let conn = Connection::open(db)?;

            Ok(conn.query_row("SELECT id, name, email FROM users WHERE email = ?1", [owned_email], |row| {
                Ok(User::new(row.get(0)?, row.get(1)?, row.get(2)?))
            })?)
        }).await?
    }

    async fn find_by_id(&self, user_id: i32) -> Result<User, QueryUserError> {
        let db = self.db_config.get_database().to_owned();
        tokio::task::spawn_blocking(move || {
            let conn = Connection::open(db)?;

            Ok(conn.query_row("SELECT id, name, email FROM users WHERE id = ?1", [user_id], |row| {
                Ok(User::new(row.get(0)?, row.get(1)?, row.get(2)?))
            })?)
        }).await?
    
    }

    async fn save_user_with_credentials(&self, user: User, password: &str) -> Result<User, UserUpdateError> {
        let db = self.db_config.get_database().to_owned();
        let owned_pass = password.to_owned();
        let user_id = tokio::task::spawn_blocking(move || {
            let mut conn = Connection::open(db)?;

            let tx = conn.transaction()?;


            let mut user_id = user.id;
            if user_id > 0 {
                let update_user = "UPDATE users SET name = ?1, email =?2 WHERE id = ?3";
                let update_creds = "UPDATE credentials SET password = ?1 WHERE user_id = ?2";
                tx.execute(update_user, (user.name, user.email, user.id))?;
                tx.execute(update_creds, (owned_pass, user.id))?;
            } else {
                let insert_user = "INSERT INTO users (name, email) values(?1, ?2)";
                let insert_creds = "INSERT INTO credentials (password, user_id) values(?1, ?2)";
                tx.execute(insert_user, (user.name, user.email))?;

                user_id = tx.last_insert_rowid() as i32;
                tx.execute(insert_creds, (owned_pass, user_id))?;
            }

            tx.commit()?;

            Ok::<i32, UserUpdateError>(user_id)
        }).await??;

        let user = self.find_by_id(user_id)
            .await
            .map_err(|_| UserUpdateError::new("Unable to retrieve user after update"))?;


        Ok(user)
        
    }

    async fn find_credentials_by_user_id(&self, user_id: i32) -> Result<Credentials, QueryUserError> {
        let db = self.db_config.get_database().to_owned();
        tokio::task::spawn_blocking(move || {
            let conn = Connection::open(db)?;

            Ok(conn.query_row("SELECT id, password, user_id FROM credentials WHERE user_id = ?1", [user_id], |row| {
                Ok(Credentials::new(row.get(0)?, row.get(1)?, row.get(2)?))
            })?)
        }).await?
    }
}
