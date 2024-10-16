use crate::db::{Comment, Post, User};
use crate::handlers::response_body::*;
use actix_web::{delete, get, patch, post, web, HttpMessage, HttpRequest, HttpResponse};
use sqlx::SqlitePool;

const UPDATE_POST: &str = "
    UPDATE post
    SET title = ?, markdown = ?, updated_at = CURRENT_TIMESTAMP
    WHERE post_id = ?
";

const ADD_POST: &str = "
    INSERT INTO post (title, markdown, user_id, created_at, updated_at)
    VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
";

const GET_POST: &str = "
    SELECT *
    FROM post
    WHERE post_id = ?
";

const DELETE_POST: &str = "
    DELETE FROM post
    WHERE post_id = ?
";

const GET_POSTS: &str = "
    SELECT *
    FROM post
    ORDER BY created_at DESC
    LIMIT ?
";

const GET_USER: &str = "
    SELECT *
    FROM user
    WHERE user_id = ?
";

const GET_POST_COMMENTS: &str = "
    SELECT *
    FROM comment
    WHERE post_id = ?
";

pub async fn check_user_owns_post(
    pool: &SqlitePool,
    user_id: i64,
    post_id: i64,
) -> Result<bool, sqlx::Error> {
    let post: Post = sqlx::query_as(GET_POST)
        .bind(post_id)
        .fetch_one(pool)
        .await?;

    Ok(post.user_id == user_id)
}

async fn get_post_comments(pool: &SqlitePool, post_id: i64) -> Result<Vec<CommentResponse>, sqlx::Error> {
    let comments: Vec<Comment> = sqlx::query_as(GET_POST_COMMENTS)
        .bind(post_id)
        .fetch_all(pool)
        .await?;

    let mut comment_responses = Vec::new();

    for comment in comments {
        let author = get_username(pool, comment.user_id).await?;

        let comment_response = CommentResponse {
            post_id: comment.post_id,
            comment_id: comment.comment_id.unwrap(),
            user_id: comment.user_id,
            created_at: comment.created_at.unwrap(),
            content: comment.content,
            author,
        };

        comment_responses.push(comment_response);
    }

    Ok(comment_responses)
}

async fn get_username(pool: &SqlitePool, user_id: i64) -> Result<String, sqlx::Error> {
    // Get username
    let user: User = sqlx::query_as(GET_USER)
        .bind(user_id)
        .fetch_one(pool)
        .await?;
    return Ok(user.username);
}

#[get("/posts")]
async fn get_posts(pool: web::Data<SqlitePool>) -> HttpResponse {
    const N: i32 = 10;
    let mut responses: Vec<PostResponse> = Vec::new();

    let posts: Vec<Post> = match sqlx::query_as(GET_POSTS)
        .bind(N)
        .fetch_all(pool.as_ref())
        .await
    {
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        Ok(posts) => posts,
    };

    for post in posts {
        let mut post_response = PostResponse {
            post_id: post.post_id.unwrap(),
            user_id: post.user_id,
            title: post.title,
            markdown: post.markdown,
            created_at: post.created_at.unwrap(),
            updated_at: post.updated_at.unwrap(),
            ..Default::default()
        };

        // Get author information
        post_response.author = match get_username(pool.as_ref(), post.user_id).await {
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            Ok(username) => username,
        };

        post_response.comments = match get_post_comments(pool.as_ref(), post.post_id.unwrap()).await {
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            Ok(coms) => coms,
        };
        responses.push(post_response);
    }

    return HttpResponse::Ok().json(responses);
}

#[get("/post/{post_id}")]
async fn get_post(pool: web::Data<SqlitePool>, post_id: web::Path<i64>) -> HttpResponse {
    let post: Post = match sqlx::query_as(GET_POST)
        .bind(post_id.into_inner())
        .fetch_one(pool.as_ref())
        .await {
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            Ok(post) => post,
        };

    let mut post_response = PostResponse {
        post_id: post.post_id.unwrap(),
        user_id: post.user_id,
        title: post.title,
        markdown: post.markdown,
        created_at: post.created_at.unwrap(),
        updated_at: post.updated_at.unwrap(),
        ..Default::default()
    };

    // Get author information
    post_response.author = match get_username(pool.as_ref(), post.user_id).await {
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        Ok(username) => username,
    };

    post_response.comments = match get_post_comments(pool.as_ref(), post.post_id.unwrap()).await {
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        Ok(coms) => coms,
    };

    return HttpResponse::Ok().json(post_response);
}

#[delete("/post/{post_id}")]
async fn delete_post(
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
    post_id: web::Path<i64>,
) -> HttpResponse {
    let post_id = post_id.into_inner();
    let ext = req.extensions();

    let user = match ext.get::<User>() {
        None => return HttpResponse::Unauthorized().body(INVALID_AUTH),
        Some(u) => u
    };

    match check_user_owns_post(&pool, user.user_id.unwrap(), post_id).await {
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        Ok(false) => return HttpResponse::Unauthorized().body(USER_MISMATCH),
        Ok(true) => {
            match sqlx::query(DELETE_POST).bind(post_id).execute(pool.as_ref()).await {
                Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
                Ok(_) => return HttpResponse::Ok().body(CONFIRM_DELETE),
            }
        }
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
    let ext = req.extensions();

    let user = match ext.get::<User>() {
        None => return HttpResponse::Unauthorized().body(INVALID_AUTH),
        Some(u) => u,
    };

    // Check if the user owns the post
    match check_user_owns_post(&pool, user.user_id.unwrap(), post_id).await {
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        Ok(false) => return HttpResponse::ImATeapot().body(USER_MISMATCH),
        Ok(true) => {

            match sqlx::query(UPDATE_POST)
                .bind(&post.title)
                .bind(&post.markdown)
                .bind(post_id)
                .execute(pool.as_ref())
                .await
            {
                Ok(_) => HttpResponse::Ok().body(CONFIRM_UPDATE),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
    }
}


#[post("/post")]
async fn add_post(
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
    post: web::Json<Post>,
) -> HttpResponse {
    let ext = req.extensions();

    let user = match ext.get::<User>() {
        None => return HttpResponse::Unauthorized().body(INVALID_AUTH),
        Some(u) => u
    };

     match sqlx::query(ADD_POST)
        .bind(&post.title)
        .bind(&post.markdown)
        .bind(user.user_id)
        .execute(pool.as_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().body(CONFIRM_INSERT),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}