use database::{db::Database, pgx::PgRow};
use logger::logger::Logger;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: String,
    name: String,
    username: String,
}

#[derive(Deserialize, Debug)]
pub struct QueryUser {
    q: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
}

pub trait UserService {
    async fn get_user_by_id(&self, id: &str) -> Result<Option<User>, String>;
    async fn get_users(&self, query: &QueryUser) -> Result<Vec<User>, String>;
    async fn update_user(&self, id: &str, user: &User) -> Result<String, String>;
}

pub struct UserServiceImpl<D: Database<PgRow>, L: Logger> {
    db: D,
    logger: L,
}

impl<D: Database<PgRow>, L: Logger> UserServiceImpl<D, L> {
    pub fn new(db: D, logger: L) -> Self {
        Self { db, logger }
    }
}

impl<D: Database<PgRow>, L: Logger> UserService for UserServiceImpl<D, L> {
    async fn get_user_by_id(&self, id: &str) -> Result<Option<User>, String> {
        let message = format!("querying user with id: {}", id);
        self.logger.info("user_service::get_user_by_id", &message);
        let row = self
            .db
            .query_one(
                "SELECT id, name, username FROM users WHERE id = $1",
                &vec![&id.to_string()],
            )
            .await;
        if row.is_ok() {
            let row = row.unwrap();
            let id = row.get(0);
            let name = row.get(1);
            let username = row.get(2);
            return Ok(Some(User {
                id: id.to_string(),
                name: name.to_string(),
                username: username.to_string(),
            }));
        } else {
            let message = format!("user with id: {} not found", id,);
            self.logger.error("user_service::get_user_by_id", &message);
            return Err(message);
        }
    }

    async fn get_users(&self, query: &QueryUser) -> Result<Vec<User>, String> {
        let limit = query.limit.unwrap_or(10);
        let offset = query.offset.unwrap_or(0);
        let mut sql = "SELECT id, name, username FROM users".to_string();
        if let Some(username) = &query.q {
            sql = format!(
                "{} WHERE username ILIKE '%{}%' OR name ILIKE '%{}%'",
                sql, username, username
            );
        }
        sql = format!("{} LIMIT {} OFFSET {}", sql, limit, offset);
        let message = format!("querying users with sql: {}", sql);
        self.logger.info("user_service::get_users", &message);
        let rows = self.db.query(&sql, &vec![]).await;
        if rows.is_ok() {
            let rows = rows.unwrap();
            let mut users: Vec<User> = Vec::new();
            for row in rows {
                let id = row.get(0);
                let name = row.get(1);
                let username = row.get(2);
                users.push(User {
                    id: id.to_string(),
                    name: name.to_string(),
                    username: username.to_string(),
                });
            }
            return Ok(users);
        } else {
            let message = format!("users not found");
            self.logger.error("user_service::get_users", &message);
            return Ok(vec![]);
        }
    }

    async fn update_user(&self, id: &str, user: &User) -> Result<String, String> {
        let message = format!("updating user with id: {}", id);
        self.logger.info("user_service::update_user", &message);
        let mut sql = "UPDATE users".to_string();
        if user.name != "" {
            sql = format!("{} SET name = $1,", sql);
        }
        if user.username != "" {
            sql = format!("{} username = $2", sql);
        }
        sql = format!("{} WHERE id = $3", sql);
        let id = id.to_string();
        let params = vec![&user.name, &user.username, &id];
        let affected_rows = self.db.execute(&sql, &params).await;
        if affected_rows.is_ok() {
            let affected_rows = affected_rows.unwrap();
            if affected_rows > 0 {
                return Ok(format!("Successfully updated user with id: {}", id));
            } else {
                return Err(format!("Failed to update user with id: {}", id));
            }
        } else {
            let message = format!("Failed to update user with id: {}", id);
            self.logger.error("user_service::update_user", &message);
            return Err(message);
        }
    }
}
