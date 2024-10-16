use crate::db::{Post, User, Comment};
use crate::handlers::response_body::{PostResponse, CommentResponse};
use actix_web::{delete, get, patch, post, web, HttpMessage, HttpRequest, HttpResponse};
use sqlx::SqlitePool;

pub async fn check_user_owns_post(
    pool: &SqlitePool,
    user_id: i64,
    post_id: i64,
) -> Result<bool, sqlx::Error> {
    let query = "SELECT user_id FROM post WHERE post_id = ?";
    let post_user_id = sqlx::query_scalar::<_, i64>(query)
        .bind(post_id)
        .fetch_one(pool)
        .await?;
    Ok(post_user_id == user_id)
}

const GET_POSTS: &str = "
    SELECT *
    FROM post
    ORDER BY created_at DESC
    LIMIT ?
";

#[get("/posts")]
async fn get_posts(
    pool: web::Data<SqlitePool>
) -> HttpResponse {
    const N: i32 = 10;
    let posts: Vec<Post> = match sqlx::query_as(GET_POSTS).bind(N).fetch_all(pool.as_ref()).await {
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        Ok(posts) => posts,
    };

    return HttpResponse::Ok().json(posts);
}

const GET_POST: &str = "";

#[get("/post/{post_id}")]
async fn get_post(
    pool: web::Data<SqlitePool>,
    post_id: web::Path<i64>
) -> HttpResponse {
    return HttpResponse::InternalServerError().body("Not implemented")
}

#[delete("/post/{post_id}")]
async fn delete_post(
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
    post_id: web::Path<i64>,
) -> HttpResponse {
    return HttpResponse::InternalServerError().body("Not implemented")
}

#[patch("/post/{post_id}")]
async fn edit_post(
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
    post_id: web::Path<i64>,
    post: web::Json<Post>,
) -> HttpResponse {
    return HttpResponse::InternalServerError().body("Not implemented")
}

#[post("/post")]
async fn add_post(
    req:HttpRequest,
    pool: web::Data<SqlitePool>,
    post: web::Json<Post>
) -> HttpResponse {
    return HttpResponse::InternalServerError().body("Not implemented")
}