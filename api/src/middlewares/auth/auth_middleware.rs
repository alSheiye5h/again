use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::sync::Arc;

use actix_web::body::EitherBody;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage, HttpResponse,
};
use sqlx::PgPool;

use crate::jwt::validate_jwt::validate_jwt;
use crate::models::jwtStruct::Keys;

pub struct RedirectIfAuthenticated {
    pub redirect_path: String,
}

impl<S, B> Transform<S, ServiceRequest> for RedirectIfAuthenticated
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = RedirectIfAuthenticatedMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RedirectIfAuthenticatedMiddleware {
            service: Arc::new(service),
            redirect_path: self.redirect_path.clone(),
        }))
    }
}

pub struct RedirectIfAuthenticatedMiddleware<S> {
    service: Arc<S>,
    redirect_path: String,
}

impl<S, B> Service<ServiceRequest> for RedirectIfAuthenticatedMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Attempt to get data from the request.
        // Clone the data pointers and values to move them into the async block.
        let token_cookie = req.cookie("j_lg_ui");
        let keys = req.app_data::<web::Data<Keys>>().cloned();
        let db_pool = req.app_data::<web::Data<PgPool>>().cloned();
        let service = self.service.clone();
        let redirect_path = self.redirect_path.clone();

        Box::pin(async move {
            if let (Some(cookie), Some(keys), Some(db_pool)) = (token_cookie, keys.as_ref(), db_pool.as_ref()) {
                let token = cookie.value();
                if let Ok(token_data) = validate_jwt(token, keys) {
                    if let Ok(user_id) = token_data.claims.sub.parse::<i32>() {
                        // Check if user exists in the database
                        let user_exists =
                            sqlx::query("SELECT id FROM users WHERE id = $1 AND deleted IS NOT TRUE")
                                .bind(user_id)
                                .fetch_optional(db_pool.get_ref())
                                .await;

                        if let Ok(Some(_)) = user_exists {
                            // User is authenticated and exists, create a redirect response.
                            let (http_req, _) = req.into_parts();
                            // let response = HttpResponse::Found()
                            //     .append_header(("Location", redirect_path.as_str()))
                            //     .finish()
                            //     .map_into_right_body(); // This is our middleware's response
                            // return Ok(ServiceResponse::new(http_req, response));
                            let response = HttpResponse::Forbidden()
                                .json(serde_json::json!({"status": "error", "message": "user already login"}))
                                .map_into_right_body(); // This is our middleware's response
                            return Ok(ServiceResponse::new(http_req, response));
                        }
                    }
                }
            }

           
    }
}
