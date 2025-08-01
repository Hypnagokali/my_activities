use std::sync::Arc;

use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHasher};
use async_trait::async_trait;
use authfix::multifactor::factor_impl::authenticator::{GetTotpSecretError, TotpSecretRepository};
use rusqlite::Connection;

use crate::{config::db::DbConfig, domain::{user::{Credentials, Mfa, User}, user_api::UserApi}, error::errors::{QueryUserError, UserUpdateError}};

pub struct UserService {
    db_config: Arc<DbConfig>
}

impl UserService {
    pub fn new(db_config: Arc<DbConfig>) -> Self {
        Self {
            db_config
        }
    }

    /// Utility method for password hashing
    pub fn hash_password(password: &str) -> Result<String, UserUpdateError> {
        let salt = SaltString::generate(&mut OsRng);

        Ok(Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| UserUpdateError::new("Cannot hash password"))?
            .to_string())
    }
}

impl From<QueryUserError> for GetTotpSecretError {
    fn from(value: QueryUserError) -> Self {
        GetTotpSecretError::new(&format!("Query user error: {}", value))
    }
}

impl TotpSecretRepository for UserService {
    type User = User;
    
    async fn auth_secret(&self, user: &User) -> Result<String, GetTotpSecretError> {
        let creds = self.find_credentials_by_user_id(user.id).await?;
    
        if let Some(config) = creds.mfa_config {
            if let Some(secret) = config.secret {
                Ok(secret)
            } else {
                log::error!("User tries to login with authenticator, but no secret was configured (User id = {})", user.id);
                Err(GetTotpSecretError::new("No TOTP secret configured"))
            }
        } else {
            log::error!("User tries to login with authenticator without mfa configured (User id = {})", user.id);
            Err(GetTotpSecretError::new("No mfa config found in users credentials"))
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

    /// Takes in plain text password
    async fn save_user_with_credentials(&self, user: User, password: &str) -> Result<User, UserUpdateError> {
        let db = self.db_config.get_database().to_owned();
        let owned_pass = password.to_owned();
        
        let user_id = tokio::task::spawn_blocking(move || {           
            let hashed_password = UserService::hash_password(&owned_pass)?;
            let mut conn = Connection::open(db)?;

            let tx = conn.transaction()?;

            let mut user_id = user.id;
            if user_id > 0 {
                let update_user = "UPDATE users SET name = ?1, email =?2 WHERE id = ?3";
                let update_creds = "UPDATE credentials SET password = ?1 WHERE user_id = ?2";
                tx.execute(update_user, (user.name, user.email, user.id))?;
                tx.execute(update_creds, (hashed_password.to_string(), user.id))?;
            } else {
                let insert_user = "INSERT INTO users (name, email) values(?1, ?2)";
                let insert_creds = "INSERT INTO credentials (password, user_id) values(?1, ?2)";
                tx.execute(insert_user, (user.name, user.email))?;

                user_id = tx.last_insert_rowid() as i32;
                tx.execute(insert_creds, (hashed_password.to_string(), user_id))?;
            }

            tx.commit()?;

            Ok::<i32, UserUpdateError>(user_id)
        }).await??;

        let user = self.find_by_id(user_id)
            .await
            .map_err(|_| UserUpdateError::new("Unable to retrieve user after update"))?;


        Ok(user)
        
    }

    /// Expects that password is already hashed
    async fn save_credentials(&self, credentials: Credentials) -> Result<Credentials, UserUpdateError> {
        if credentials.user_id == 0 {
            Err(UserUpdateError::new("Cannot save credentials if user_id is 0"))
        } else {
            let db = self.db_config.get_database().to_owned();

            let mfa_config = match credentials.mfa_config {
                Some(mfa_config) => {
                    let secret = match mfa_config.secret {
                        Some(secret) => secret,
                        None => "null".to_owned(),
                    };

                    (mfa_config.mfa_id, secret)
                },
                None => todo!(),
            };

            let command = match credentials.id > 0 {
                true => ("UPDATE credentials SET password = ?1, mfa_id = ?2, mfa_secret = ?3 WHERE id = ?4", 
                    (credentials.password, mfa_config.0, mfa_config.1, credentials.id)),
                false => ("INSERT INTO credentials (password, mfa_id, mfa_secret, user_id) values (?1, ?2, ?3, ?4)", 
                    (credentials.password, mfa_config.0, mfa_config.1, credentials.user_id)),
            };

            let exec: Result<(), rusqlite::Error> = tokio::task::spawn_blocking(move || {
                let conn: Connection = Connection::open(db)?;    
                conn.execute(command.0, command.1)?;

                Ok::<(), rusqlite::Error>(())
            }).await?;

            match exec {
                Ok(_) => self.find_credentials_by_user_id(credentials.user_id).await
                .map_err(|e| UserUpdateError::new(&format!("Cannot load credentials after save: {}", e))),
                Err(e) => Err(UserUpdateError::new(&format!("Cannot insert or update credentials: {}", e))),
            }
        }   
    }

    async fn find_credentials_by_user_id(&self, user_id: i32) -> Result<Credentials, QueryUserError> {
        let db = self.db_config.get_database().to_owned();
        tokio::task::spawn_blocking(move || {
            let conn = Connection::open(db)?;

            Ok(conn.query_row("SELECT id, password, mfa_id, mfa_secret, user_id FROM credentials WHERE user_id = ?1", [user_id], |row| {
                let mfa_id: Option<String> = row.get(2)?;
                let mfa_secret: Option<String> = row.get(3)?;

                let mut mfa_config = None;
                if let Some(mfa_id) = mfa_id {
                    if let Some(mfa_secret) = mfa_secret {
                        mfa_config = Some(Mfa::with_secret(&mfa_id, &mfa_secret));
                    } else {
                        mfa_config = Some(Mfa::new(&mfa_id));
                    }
                }

                let mut creds = Credentials::new(row.get(0)?, row.get(1)?, row.get(4)?);
                if let Some(mfa_config) = mfa_config {
                    creds.set_mfa(mfa_config);
                }
                Ok(creds)
            })?)
        }).await?
    }
}


#[cfg(test)]
mod user_service_tests {
    use std::sync::Arc;

    use crate::{config::db::DbConfig, create_db, domain::{user::{Mfa, User}, user_api::UserApi}, service::user_service::UserService};


    #[tokio::test]
    async fn should_be_able_to_save_credentials() {
        let temp_db = "file:user_service_test?mode=memory&cache=shared";
        let db_config = DbConfig::new(temp_db);
        let _db = create_db(&db_config);

        // Arrange
        let user_service = UserService::new(Arc::new(db_config));
        let user = User::new(0, "test@example.org".to_owned(), "Test User".to_owned());
        let saved_user = user_service.save_user_with_credentials(user, "secretpassword").await.unwrap();


        // Act
        let mut creds = user_service.find_credentials_by_user_id(saved_user.id).await.unwrap();
        creds.set_mfa(Mfa::with_secret("MFA_ID", "asecret"));
        user_service.save_credentials(creds).await.unwrap();

        // Assert
        creds = user_service.find_credentials_by_user_id(saved_user.id).await.unwrap();

        assert!(creds.mfa_config.is_some());
        let mfa_config = creds.mfa_config.unwrap();
        assert_eq!(mfa_config.mfa_id, "MFA_ID");
        assert!(mfa_config.secret.is_some());
        assert_eq!(mfa_config.secret.unwrap(), "asecret");
    }

}
