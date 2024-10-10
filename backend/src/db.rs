use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{migrate::MigrateDatabase, Error, Sqlite, SqlitePool};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Default)]
pub struct Post {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub title: String,
    pub markdown: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Default)]
pub struct Comment {
    pub id: Option<i64>,
    pub post_id: i64,
    pub user_id: Option<i64>,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
}

pub async fn init(db_url: &str) -> Result<SqlitePool, Error> {
    match Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        false => {
            println!("Creating Sqlite database");
            match Sqlite::create_database(&db_url).await {
                Ok(_) => println!("Database successfully created"),
                Err(e) => return Err(e),
            }
        }
        true => {
            println!("Sqlite database exists");
        }
    }

    let db = SqlitePool::connect(&db_url).await?;

    let enable_foreign_key_result = sqlx::query(ENABLE_FOREIGN_KEY).execute(&db).await?;
    println!("Enable foreign key result: {:?}", enable_foreign_key_result);

    let post_table_result = sqlx::query(CREATE_POST_TABLE).execute(&db).await?;
    println!("Create post table result: {:?}", post_table_result);
    let comment_table_result = sqlx::query(CREATE_COMMENT_TABLE).execute(&db).await?;
    println!("Create comment table result: {:?}", comment_table_result);
    let user_table_result = sqlx::query(CREATE_USER_TABLE).execute(&db).await?;
    println!("Create user table result: {:?}", user_table_result);

    println!("Database creation finished");
    Ok(db)
}

pub const ENABLE_FOREIGN_KEY: &str = "PRAGMA foreign_keys = ON;";

pub const CREATE_POST_TABLE: &str = "CREATE TABLE IF NOT EXISTS post (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title VARCHAR(255) NOT NULL,
    markdown TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
)";

pub const CREATE_COMMENT_TABLE: &str = "CREATE TABLE IF NOT EXISTS comment (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    post_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_post FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
)";

pub const CREATE_USER_TABLE: &str = "CREATE TABLE IF NOT EXISTS user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(320) NOT NULL UNIQUE,
    password BINARY(60) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)";

