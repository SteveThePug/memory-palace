use crate::auth::make_token;
use crate::db::User;
use actix_web::{delete, get, post, web, HttpMessage, HttpRequest, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::SqlitePool;

use crate::handlers::response_body::*;

const N: u32 = 10;
const GET_USERS: &str = "SELECT * FROM user LIMIT ?";
const GET_USER_BY_USERNAME: &str = "SELECT * FROM user WHERE username = ?";
const SIGN_IN: &str = "SELECT * FROM user WHERE username = ? OR email = ?";
const SIGN_UP: &str = "
    INSERT INTO user (username, email, password, created_at)
    VALUES (?, ?, ?, CURRENT_TIMESTAMP)
";

const DELETE_USER: &str = "
    DELETE FROM user
    WHERE user_id = ?
";

const GET_USER: &str = "
    SELECT *
    FROM user
    WHERE user_id = ?
";

pub async fn get_username(pool: &SqlitePool, user_id: i64) -> Result<String, sqlx::Error> {
    // Get username
    let user: User = sqlx::query_as(GET_USER)
        .bind(user_id)
        .fetch_one(pool)
        .await?;
    return Ok(user.username);
}

#[get("/users")]
async fn get_users(pool: web::Data<SqlitePool>) -> HttpResponse {
    let users: Vec<User> = match sqlx::query_as(GET_USERS)
        .bind(N)
        .fetch_all(pool.as_ref())
        .await
    {
        Ok(users) => users,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let response: Vec<UserResponse> = users
        .into_iter()
        .map(|user| UserResponse {
            user_id: user.user_id.unwrap(),
            username: user.username,
            created_at: user.created_at.unwrap(), // Assuming created_at is an Option<NaiveDateTime>
        })
        .collect();

    HttpResponse::Ok().json(response)
}

#[get("/user/{username}")]
async fn get(pool: web::Data<SqlitePool>, username: web::Path<String>) -> HttpResponse {
    let user: User = match sqlx::query_as(GET_USER_BY_USERNAME)
        .bind(username.into_inner())
        .fetch_one(pool.as_ref())
        .await
    {
        Ok(user) => user,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let response: UserResponse = UserResponse {
        user_id: user.user_id.unwrap(),
        username: user.username,
        created_at: user.created_at.unwrap(),
    };

    HttpResponse::Ok().json(response)
}

#[post("/user/signin")]
async fn sign_in(pool: web::Data<SqlitePool>, user: web::Json<User>) -> HttpResponse {
    let db_user: User = match sqlx::query_as(SIGN_IN)
        .bind(&user.username)
        .bind(&user.email)
        .fetch_one(pool.as_ref())
        .await
    {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().body(INVALID_AUTH),
    };

    if verify(&user.password, &db_user.password).unwrap_or(false) {
        match make_token(&db_user) {
            Ok(token) => HttpResponse::Ok().body(token),
            Err(_) => HttpResponse::InternalServerError().body(TOKEN_FAILURE),
        }
    } else {
        HttpResponse::Unauthorized().body(INVALID_AUTH)
    }
}

#[post("/user/signup")]
async fn sign_up(pool: web::Data<SqlitePool>, user: web::Json<User>) -> HttpResponse {
    let hashed_password = match hash(&user.password, DEFAULT_COST) {
        Ok(p) => p,
        Err(_) => return HttpResponse::InternalServerError().body(PASSWORD_FAILURE),
    };

    match sqlx::query(SIGN_UP)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&hashed_password)
        .execute(pool.as_ref())
        .await
    {
        Ok(_) => {
            let db_user: User = match sqlx::query_as(GET_USER_BY_USERNAME)
                .bind(&user.username)
                .fetch_one(pool.as_ref())
                .await
            {
                Ok(user) => user,
                Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            };

            match make_token(&db_user) {
                Ok(token) => HttpResponse::Ok().body(token),
                Err(_) => HttpResponse::InternalServerError().body(TOKEN_FAILURE),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/user")]
async fn delete(pool: web::Data<SqlitePool>, req: HttpRequest) -> HttpResponse {
    let ext = req.extensions();

    let user = match ext.get::<User>() {
        None => return HttpResponse::Unauthorized().body(INVALID_AUTH),
        Some(u) => u,
    };

    match sqlx::query(DELETE_USER)
        .bind(user.user_id)
        .execute(pool.as_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().body(CONFIRM_DELETE),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
