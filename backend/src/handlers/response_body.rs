use crate::db::{Post, Comment, User};
use serde::{Serialize, Deserialize};

pub const CONFIRM_DELETE: &str = "DELETE success";
pub const CONFIRM_UPDATE: &str = "UPDATE success";
pub const CONFIRM_INSERT: &str = "INSERT success";

pub const USER_NOT_FOUND: &str = "User not found";
pub const POST_NOT_FOUND: &str = "Post not found";
pub const COMMENT_NOT_FOUND: &str = "Comment not found";

pub const INVALID_AUTH: &str = "Authorization not accepted";
pub const INVALID_PASSWORD: &str = "Passwords don't match";

pub const NO_USER_ID: &str = "No used ID found";
pub const USER_MISMATCH: &str = "Users mismatch";

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Default)]
pub struct PostResponse {
    pub id: i64,
    pub username: String,
    pub user_id: i64,
    pub markdown: String,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub comments: Vec<CommentResponse>
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Default)]
pub struct CommentResponse {
    pub id: i64,
    pub username: String,
    pub user_id: i64,
    pub author: String,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Default)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub user: User,
    pub token: String
}