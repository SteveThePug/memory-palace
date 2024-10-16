use crate::auth::make_token;
use crate::db::User;
use actix_web::{delete, get, post, web, HttpMessage, HttpRequest, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::handlers::response_body::*;
use sqlx::SqlitePool;


const GET_USERS: &str = "SELECT * FROM user LIMIT ?";

#[get("/users")]
async fn get_users(
    pool: web::Data<SqlitePool>
) -> HttpResponse {
    return HttpResponse::InternalServerError().body("Not implemented");
}

const GET_USER: &str = "SELECT * FROM user WHERE username = ?";

/// Gets the user with the supplied username.
#[get("/user/{username}")]
async fn get(pool: web::Data<SqlitePool>, username: web::Path<String>) -> HttpResponse {
    return HttpResponse::InternalServerError().body("Not implemented");
}

const SIGN_IN: &str = "SELECT * FROM user WHERE username = ? OR email = ?";

#[post("/user/signin")]
async fn sign_in(pool: web::Data<SqlitePool>, user: web::Json<User>) -> HttpResponse {
    return HttpResponse::InternalServerError().body("Not implemented");
}

const SIGN_UP: &str = "";

#[post("/user/signup")]
async fn sign_up(pool: web::Data<SqlitePool>, user: web::Json<User>) -> HttpResponse {
    return HttpResponse::InternalServerError().body("Not implemented");
}

#[delete("/user")]
async fn delete(
    pool: web::Data<SqlitePool>,
    req: HttpRequest
) -> HttpResponse {
    return HttpResponse::InternalServerError().body("Not implemented");
}
