use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, FromRequest, HttpMessage,
};
use crate::{jwt::validate_jwt::validate_jwt, models::jwtStruct::{Claims, Keys}};

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract the token from the "j_lg_ui" cookie
        let token = match req.cookie("j_lg_ui") {
            Some(cookie) => cookie.value().to_string(),
            None => {
                // If the cookie is not found, this is an unauthorized request.
                // We can't return a future with an error directly here in a simple way
                // with this middleware structure. A more common approach is to extract
                // an identity and let the handler decide, or use a different middleware pattern.
                // For now, we'll proceed and let validation fail.
                String::new()
            }
        };

        // Extract Keys from app data
        if let Some(keys) = req.app_data::<web::Data<Keys>>() {
            match validate_jwt(&token, keys) {
                Ok(token_data) => {
                    // The token is valid.
                    // We can insert the user_id (from token_data.claims.sub) into request extensions
                    // for handlers to use.
                    req.extensions_mut().insert(token_data.claims);
                }
                Err(_) => {
                    // Token is invalid or not present.
                    // We don't add anything to extensions. The handler will have to check.
                    // A stricter middleware would return an error response here.
                }
            }
        }

        self.service.call(req)
    }
}

// This is a helper struct to extract the user ID in your handlers
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let extensions = req.extensions();
        let claims = extensions.get::<Claims>();

        match claims {
            Some(claims) => ready(Ok(AuthenticatedUser {
                id: claims.sub.clone(),
            })),
            None => ready(Err(actix_web::error::ErrorUnauthorized(
                "Not authenticated",
            ))),
        }
    }
}
