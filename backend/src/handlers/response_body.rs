use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

pub const CONFIRM_DELETE: &str = "DELETE success";
pub const CONFIRM_UPDATE: &str = "UPDATE success";
pub const CONFIRM_INSERT: &str = "INSERT success";

// pub const USER_NOT_FOUND: &str = "User not found";
// pub const POST_NOT_FOUND: &str = "Post not found";
pub const COMMENT_NOT_FOUND: &str = "Comment not found";

pub const INVALID_AUTH: &str = "Authorization not accepted";
// pub const INVALID_PASSWORD: &str = "Passwords don't match";

pub const TOKEN_FAILURE: &str = "Token could not be created";
pub const PASSWORD_FAILURE: &str = "Password could not be hashed";

// pub const NO_USER_ID: &str = "No used ID found";
pub const USER_MISMATCH: &str = "Users mismatch";

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Default)]
pub struct PostResponse {
    // Included in the Post table
    pub post_id: i64,
    pub user_id: i64,
    pub title: String,
    pub markdown: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // Retrieve from User table on user_id
    pub author: String,
    // Retrieve from Comment table on post_id
    pub comments: Vec<CommentResponse>
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Default)]
pub struct CommentResponse {
    // Included in the Comment table
    pub comment_id: i64,
    pub post_id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_at: NaiveDateTime,
    // Retrieve from User table from user_id
    pub author: String
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Default)]
pub struct UserResponse {
    // Included in User table
    pub user_id: i64,
    pub username: String,
    pub created_at: NaiveDateTime
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub user: UserResponse,
    pub token: String
}