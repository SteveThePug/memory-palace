use actix_web::{
    body::BoxBody, dev::{ServiceRequest, ServiceResponse}, http::header::AUTHORIZATION, middleware::Next, Error, HttpMessage, HttpResponse
};
use crate::db::User;
use std::env;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData};

pub fn make_token(
    content: &User
) -> Result<String, jsonwebtoken::errors::Error> {
    // Generate JWT token
    let secret = env::var("STP_SECRET").expect("Failed to get environment variable STP_SECRET");
    let token = encode(&Header::default(), content, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok(token)
}

pub fn decode_token(
    token: &str
) -> Result<TokenData<User>, jsonwebtoken::errors::Error> {
    // Verify JWT token
    let secret = env::var("STP_SECRET").expect("Failed to get environment variable STP_SECRET");
    let mut validation = jsonwebtoken::Validation::default();
    validation.validate_exp = false;
    validation.set_required_spec_claims(&[""]);
    let token_data = decode(&token, &DecodingKey::from_secret(secret.as_ref()), &validation)?;
    Ok(token_data)
}

pub async fn verify_token(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    // Pre-processing: Checking Authorization header
    if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            println!("{}!", auth_str);
            // Ensure the header starts with "Bearer "
            if auth_str.starts_with("Bearer ") {
                // Extract the token by removing the "Bearer " prefix
                let token = &auth_str[7..];

                // Decode the token
                if let Ok(claims) = decode_token(token) {
                    let user = claims.claims;

                    // Attach the user info to the request extensions for use in the next service
                    req.extensions_mut().insert(user);
                    return next.call(req).await;
                }
            }
        }
    }

    // If token is invalid or missing, return Unauthorized response
    Ok(req.into_response(HttpResponse::Unauthorized().finish().map_into_boxed_body()))
}
