use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{migrate::MigrateDatabase, Error, Sqlite, SqlitePool};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Default)]
pub struct Post {
    pub post_id: Option<i64>,
    pub user_id: Option<i64>,
    pub title: String,
    pub markdown: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Default)]
pub struct Comment {
    pub comment_id: Option<i64>,
    pub post_id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub user_id: Option<i64>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
}

pub async fn init(db_url: &str) -> Result<SqlitePool, Error> {
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating Sqlite database");
        if let Err(e) = Sqlite::create_database(db_url).await {
            return Err(e);
        }
        println!("Database successfully created");
    } else {
        println!("Sqlite database exists");
    }

    let db = SqlitePool::connect(db_url).await?;

    sqlx::query(ENABLE_FOREIGN_KEY)
        .execute(&db)
        .await
        .map(|result| println!("Enable foreign key result: {:?}", result))?;

    sqlx::query(CREATE_POST_TABLE)
        .execute(&db)
        .await
        .map(|result| println!("Create post table result: {:?}", result))?;

    sqlx::query(CREATE_COMMENT_TABLE)
        .execute(&db)
        .await
        .map(|result| println!("Create comment table result: {:?}", result))?;

    sqlx::query(CREATE_USER_TABLE)
        .execute(&db)
        .await
        .map(|result| println!("Create user table result: {:?}", result))?;

    println!("Database creation finished");
    Ok(db)
}

pub const ENABLE_FOREIGN_KEY: &str = "PRAGMA foreign_keys = ON;";

pub const CREATE_POST_TABLE: &str = "CREATE TABLE IF NOT EXISTS post (
    post_id INTEGER PRIMARY KEY AUTOINCREMENT,
    title VARCHAR(255) NOT NULL,
    markdown TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES user(user_id) ON DELETE CASCADE
)";

pub const CREATE_COMMENT_TABLE: &str = "CREATE TABLE IF NOT EXISTS comment (
    comment_id INTEGER PRIMARY KEY AUTOINCREMENT,
    post_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_post FOREIGN KEY (post_id) REFERENCES post(post_id) ON DELETE CASCADE,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES user(user_id) ON DELETE CASCADE
)";

pub const CREATE_USER_TABLE: &str = "CREATE TABLE IF NOT EXISTS user (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(320) NOT NULL UNIQUE,
    password BINARY(60) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)";