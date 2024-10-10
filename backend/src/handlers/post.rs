use crate::db::{Post, User, Comment};
use crate::handlers::response_body::*;
use actix_web::{delete, get, patch, post, web, HttpMessage, HttpRequest, HttpResponse};
use sqlx::SqlitePool;

pub async fn check_user_owns_post(
    pool: &SqlitePool,
    user_id: i64,
    post_id: i64,
) -> Result<bool, sqlx::Error> {

    let query = "SELECT user_id FROM post WHERE id = ?";
    let post_user_id = sqlx::query_scalar::<_, i64>(query)
        .bind(post_id)
        .fetch_one(pool)
        .await?;
    Ok(post_user_id == user_id)
}

#[get("/posts")]
async fn get_posts(
    pool: web::Data<SqlitePool>
) -> HttpResponse {
let query = "
        SELECT 
            post.id AS post_id,
            post.title,
            post.markdown,
            post.created_at AS post_created_at,
            post.updated_at AS post_updated_at,
            author.username AS author_name,
            author.id AS author_id,
            comment.id AS comment_id,
            comment.content AS comment_content,
            comment.created_at AS comment_created_at,
            commenter.username AS commenter_name,
            comment.user_id AS commenter_id
        FROM post
        JOIN user AS author ON post.user_id = author.id
        LEFT JOIN comment ON post.id = comment.post_id
        LEFT JOIN user AS commenter ON comment.user_id = commenter.id
        ORDER BY post.created_at DESC, comment.created_at ASC;
";

match sqlx::query(query)
    .map(|row| {
        // Map fields for PostResponse and CommentResponse
        let post_id: i64 = row.get("post_id");
        let post = PostResponse {
            id: post_id,
            username: row.get("author_name"),
            user_id: row.get("author_id"),
            markdown: row.get("markdown"),
            title: row.get("title"),
            created_at: row.get("post_created_at"),
            updated_at: row.get("post_updated_at"),
            comments: Vec::new(),  // Fill in comments later
        };
        
        let comment = CommentResponse {
            id: row.get("comment_id"),
            post_id,
            user_id: row.get("commenter_id"),
            username: row.get("commenter_name"),
            content: row.get("comment_content"),
            created_at: row.get("comment_created_at"),
        };
        
        (post, comment)
    })
    .fetch_all(pool.get_ref())
    .await
{
    Ok(rows) => {
        let mut posts_map = std::collections::HashMap::new();
        
        for (post, comment) in rows {
            posts_map
                .entry(post.id)
                .or_insert_with(|| PostResponse {
                    id: post.id,
                    username: post.username.clone(),
                    user_id: post.user_id,
                    markdown: post.markdown.clone(),
                    title: post.title.clone(),
                    created_at: post.created_at,
                    updated_at: post.updated_at,
                    comments: Vec::new(),
                })
                .comments
                .push(comment);
        }

        let posts_with_comments: Vec<PostResponse> = posts_map.into_values().collect();
        HttpResponse::Ok().json(posts_with_comments)
    }
    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
}


}

#[get("/post/{post_id}")]
async fn get_post(
    pool: web::Data<SqlitePool>,
    post_id: web::Path<i64>
) -> HttpResponse {
    let query = "SELECT * FROM post WHERE id = ?";
    match sqlx::query_as::<_, Post>(query)
        .bind(post_id.into_inner())
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/post/{post_id}")]
async fn delete_post(
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
    post_id: web::Path<i64>,
) -> HttpResponse {
    let post_id = post_id.into_inner();
    let extention = req.extensions();

    let user = match extention.get::<User>() {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().body(INVALID_AUTH),
    };

    let user_id = match user.id {
        Some(user_id) => user_id,
        None => return HttpResponse::Unauthorized().body(NO_USER_ID)
    };

    match check_user_owns_post(pool.get_ref(), user_id, post_id).await {
        Ok(false) => return HttpResponse::Unauthorized().body(USER_MISMATCH),
        Err(sqlx::Error::RowNotFound) => return HttpResponse::NotFound().body(POST_NOT_FOUND),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
        _ => ()
    };

    let delete_query = "DELETE FROM post WHERE id = ?";
    match sqlx::query(delete_query)
        .bind(post_id)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().body(CONFIRM_DELETE),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[patch("/post/{post_id}")]
async fn edit_post(
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
    post_id: web::Path<i64>,
    post: web::Json<Post>,
) -> HttpResponse {
    let post_id = post_id.into_inner();
    let extentions = req.extensions();

    let user = match extentions.get::<User>() {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().body(INVALID_AUTH),
    };

    let user_id = match user.id {
        Some(user_id) => user_id,
        None => return HttpResponse::Unauthorized().body(NO_USER_ID)
    };


    match check_user_owns_post(pool.get_ref(), user_id, post_id).await {
        Ok(false) => return HttpResponse::Unauthorized().body(USER_MISMATCH),
        Err(sqlx::Error::RowNotFound) => return HttpResponse::NotFound().body(POST_NOT_FOUND),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
        _ => ()
    };

    let update_query = "UPDATE post SET title = ?, markdown = ? WHERE id = ?";
    match sqlx::query(update_query)
        .bind(&post.title)
        .bind(&post.markdown)
        .bind(post_id)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().body(CONFIRM_UPDATE),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/post")]
async fn add_post(
    req:HttpRequest,
    pool: web::Data<SqlitePool>,
    post: web::Json<Post>
) -> HttpResponse {

    let extentions = req.extensions();

    let user = match extentions.get::<User>() {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().body(INVALID_AUTH),
    };

    let user_id = match user.id {
        Some(user_id) => user_id,
        None => return HttpResponse::Unauthorized().body(NO_USER_ID)
    };

    let insert_query = "INSERT INTO post (user_id, title, markdown) VALUES (?, ?, ?)";
    match sqlx::query(insert_query)
        .bind(user_id)
        .bind(&post.title)
        .bind(&post.markdown)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().body(CONFIRM_INSERT),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}