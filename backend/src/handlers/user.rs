use crate::auth::make_token;
use crate::db::User;
use actix_web::{delete, get, post, web, HttpMessage, HttpRequest, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::handlers::response_body::*;
use sqlx::SqlitePool;


#[get("/users")]
async fn get_users(
    pool: web::Data<SqlitePool>
) -> HttpResponse {
    let query = "SELECT * FROM user LIMIT 10";
    match sqlx::query_as::<_, User>(query)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/// Gets the user with the supplied username.
#[get("/user/{username}")]
async fn get(pool: web::Data<SqlitePool>, username: web::Path<String>) -> HttpResponse {
    let query = "SELECT * FROM user WHERE username = ?";
    match sqlx::query_as::<_, User>(query)
        .bind(username.into_inner())
        .fetch_one(pool.as_ref())
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/user/signin")]
async fn sign_in(pool: web::Data<SqlitePool>, user: web::Json<User>) -> HttpResponse {
    let query = "SELECT * FROM user WHERE username = ? OR email = ?";
    let user = user.into_inner();
    match sqlx::query_as::<_, User>(query)
        .bind(&user.username)
        .bind(&user.email)
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(stored_user) => {
            if verify(&user.password, &stored_user.password).unwrap_or(false) {
                match make_token(&stored_user) {
                    Ok(token) => {
                        let token_response = TokenResponse{user: stored_user, token};
                        HttpResponse::Ok().json(token_response)
                    }
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                }
            } else {
                HttpResponse::Unauthorized().body(INVALID_PASSWORD)
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/user/signup")]
async fn sign_up(pool: web::Data<SqlitePool>, user: web::Json<User>) -> HttpResponse {
    let user = user.into_inner();

    let hashed_password = match hash(&user.password, DEFAULT_COST) {
        Ok(hp) => hp,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let query = "INSERT INTO user (username, password, email) VALUES (?, ?, ?)";
    match sqlx::query(query)
        .bind(&user.username)
        .bind(&hashed_password)
        .bind(&user.email)
        .execute(pool.get_ref())
        .await
    {
        Ok(res) => {
            let inserted_id = res.last_insert_rowid();
            let query = "SELECT * FROM user WHERE id = ?";
            match sqlx::query_as::<_, User>(query)
                .bind(inserted_id)
                .fetch_one(pool.get_ref())
                .await
            {
                Ok(stored_user) => {
                    match make_token(&stored_user) {
                        Ok(token) => {
                            let token_response = TokenResponse{user, token};
                            HttpResponse::Ok().json(token_response)
                        }
                        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                    }
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/user")]
async fn delete(
    pool: web::Data<SqlitePool>,
    req: HttpRequest
) -> HttpResponse {
    let extentions = req.extensions();

    let user = match extentions.get::<User>() {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().body(INVALID_AUTH),
    };

    let user_id = match user.id {
        Some(user_id) => user_id,
        None => return HttpResponse::Unauthorized().body(NO_USER_ID),
    };

    let query = "DELETE FROM user WHERE id = ?";
    match sqlx::query(query)
        .bind(user_id)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().body(CONFIRM_DELETE),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
