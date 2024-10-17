use crate::db::{Comment, User};
use crate::handlers::response_body::*;
use crate::handlers::user::get_username;
use actix_web::{delete, get, patch, post, web, HttpMessage, HttpRequest, HttpResponse};
use sqlx::SqlitePool;

const N: u32 = 10;
const GET_COMMENTS: &str = "SELECT * FROM comment LIMIT ?";
const GET_COMMENT: &str = "SELECT * FROM comment WHERE comment_id = ?";

async fn check_user_owns_comment(
    pool: &SqlitePool,
    user_id: i64,
    comment_id: i64,
) -> Result<bool, sqlx::Error> {
    let comment: Comment = sqlx::query_as(GET_COMMENT)
        .bind(comment_id)
        .fetch_one(pool)
        .await?;
    Ok(user_id == comment.user_id)
}

#[get("/comments")]
async fn get_comments(pool: web::Data<SqlitePool>) -> HttpResponse {
    let comments: Vec<Comment> = match sqlx::query_as(GET_COMMENTS)
        .bind(N)
        .fetch_all(pool.as_ref())
        .await
    {
        Ok(comments) => comments,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let mut response: Vec<CommentResponse> = Vec::new();
    for comment in comments {
        let mut comment_response = CommentResponse {
            comment_id: comment.comment_id.unwrap(),
            post_id: comment.post_id,
            user_id: comment.user_id, // Assuming created_at is an Option<NaiveDateTime>
            created_at: comment.created_at.unwrap(),
            content: comment.content,
            author: String::new(), // Placeholder for author, will be set later
        };

        comment_response.author = match get_username(pool.as_ref(), comment.user_id).await {
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            Ok(username) => username,
        };
        response.push(comment_response);
    }

    HttpResponse::Ok().json(response)
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
        Ok(result) => {
            let comment_id = result.last_insert_rowid();
            let inserted_comment: Comment = match sqlx::query_as(GET_COMMENT)
                .bind(comment_id)
                .fetch_one(pool.as_ref())
                .await
            {
                Ok(comment) => comment,
                Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            };

            let comment_response = CommentResponse {
                comment_id: inserted_comment.comment_id.unwrap(),
                post_id: inserted_comment.post_id,
                user_id: inserted_comment.user_id,
                content: inserted_comment.content.clone(),
                created_at: inserted_comment.created_at.unwrap(),
                author: user.username.clone(),
            };

            HttpResponse::Ok().json(comment_response)
        }
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

    match check_user_owns_comment(pool.get_ref(), user_id.unwrap(), comment_id).await {
        Ok(false) => return HttpResponse::Unauthorized().body(USER_MISMATCH),
        Err(sqlx::Error::RowNotFound) => return HttpResponse::NotFound().body(COMMENT_NOT_FOUND),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
        _ => (),
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

    match check_user_owns_comment(pool.get_ref(), user_id.unwrap(), comment_id).await {
        Ok(false) => return HttpResponse::Unauthorized().body(USER_MISMATCH),
        Err(sqlx::Error::RowNotFound) => return HttpResponse::NotFound().body(COMMENT_NOT_FOUND),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
        Ok(true) => (),
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
