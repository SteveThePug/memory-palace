mod handlers {
    pub mod post;
    pub mod user;
    pub mod comment;
    pub mod response_body;
}
pub mod auth;
pub mod db;

use actix_web::middleware::{self};
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlers::{comment, post, user};
use actix_cors::Cors;
use actix_web::http;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = std::env::var("DB_URL").expect("DB_URL must be set");

    let db = match db::init(&db_url).await {
        Ok(pool) => pool,
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ));
        }
    };

    let ip = "127.0.0.1";
    let port = 8080;
    println!("Hosting server on http://{}:{}", ip, port);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")  // Replace with your frontend URL
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .service(post::get_post)
            .service(post::get_posts)
            .service(user::get)
            .service(user::get_users)
            .service(user::sign_up)
            .service(user::sign_in)
            .service(
                web::scope("")
                    .wrap(middleware::from_fn(auth::verify_token))
                    .service(user::delete)
                    .service(comment::add_comment)
                    .service(comment::edit_comment)
                    .service(comment::delete_comment)
                    .service(post::add_post)
                    .service(post::edit_post)
                    .service(post::delete_post)
            )
    })
    .bind((ip, port))?
    .run()
    .await
}

