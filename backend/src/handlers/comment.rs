use crate::db::{Comment, User};
use crate::handlers::response_body::*;
use actix_web::{delete, patch, post, web, HttpMessage, HttpRequest, HttpResponse};
use sqlx::SqlitePool;

async fn check_user_owns_comment(
    pool: &SqlitePool,
    user_id: i64,
    comment_id: i64,
) -> Result<bool, sqlx::Error> {
    let query = "SELECT user_id FROM comment WHERE id = ?";
    let comment_user_id = sqlx::query_scalar::<_, i64>(query)
        .bind(comment_id)
        .fetch_one(pool)
        .await?;
    Ok(user_id == comment_user_id)
}

#[post("/comment")]
async fn add_comment(
    pool: web::Data<SqlitePool>,
    req: HttpRequest,
    comment: web::Json<Comment>,
) -> HttpResponse {
    let extentions = req.extensions();

    let user = match extentions.get::<User>() {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().body(INVALID_AUTH),
    };

    let user_id = user.user_id;

    let insert_query = "INSERT INTO comment (post_id, user_id, content) VALUES (?, ?, ?)";
    match sqlx::query(insert_query)
        .bind(comment.post_id)
        .bind(user_id)
        .bind(&comment.content)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().body(CONFIRM_INSERT),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/comment/{commentId}")]
async fn delete_comment(
    pool: web::Data<SqlitePool>,
    req: HttpRequest,
    comment_id: web::Path<i64>,
) -> HttpResponse {
    let extentions = req.extensions();
    let comment_id = comment_id.into_inner();

    let user = match extentions.get::<User>() {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().body(INVALID_AUTH),
    };

    let user_id = user.user_id;

    match check_user_owns_comment(pool.get_ref(), user_id, comment_id).await {
        Ok(false) => return HttpResponse::Unauthorized().body(USER_MISMATCH),
        Err(sqlx::Error::RowNotFound) => return HttpResponse::NotFound().body(COMMENT_NOT_FOUND),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
        _ => ()
    }

    let delete_query = "DELETE FROM comment WHERE id = ?";
    match sqlx::query(delete_query)
        .bind(comment_id)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().body(CONFIRM_DELETE),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[patch("/comment/{commentId}")]
async fn edit_comment(
    pool: web::Data<SqlitePool>,
    req: HttpRequest,
    comment_id: web::Path<i64>,
    comment: web::Json<Comment>,
) -> HttpResponse {
    let comment_id = comment_id.into_inner();
    let extentions = req.extensions();

    let user = match extentions.get::<User>() {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().body(INVALID_AUTH),
    };

    let user_id = user.user_id;

    match check_user_owns_comment(pool.get_ref(), user_id, comment_id).await {
        Ok(false) => return HttpResponse::Unauthorized().body(USER_MISMATCH),
        Err(sqlx::Error::RowNotFound) => return HttpResponse::NotFound().body(COMMENT_NOT_FOUND),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
        Ok(true) => ()
    };

    let update_query = "UPDATE comment SET content = ? WHERE id = ?";
    match sqlx::query(update_query)
        .bind(&comment.content)
        .bind(comment_id)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().body(CONFIRM_UPDATE),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
