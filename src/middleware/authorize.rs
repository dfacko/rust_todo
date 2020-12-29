use std::task::{Context, Poll};

use crate::jwt::*;
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpMessage, HttpResponse};
use futures::future::{ok, Either, Ready};
use serde_json::json;

pub struct CheckLogin;

impl<S, B> Transform<S> for CheckLogin
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckLoginMiddleware { service })
    }
}
pub struct CheckLoginMiddleware<S> {
    service: S,
}

impl<S, B> Service for CheckLoginMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if req.path() == "/login" || req.path() == "/register" {
            return Either::Left(self.service.call(req));
        };
        let jwt = match req.headers().get("Authorization") {
            Some(header) => match header.to_str().ok() {
                Some(val) => val.to_string(),
                None => {
                    return Either::Right(ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(json!({
                                "message":"Could not parse header"
                            }))
                            //.header(http::header::LOCATION, "/login")
                            //.finish()
                            .into_body(),
                    )));
                }
            },
            None => {
                return Either::Right(ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(json!({
                            "message":"no auth header found"
                        }))
                        //.header(http::header::LOCATION, "/login")
                        //.finish()
                        .into_body(),
                )));
            }
        };
        match verify(jwt) {
            Ok(user) => {
                req.extensions_mut().insert(user);
                return Either::Left(self.service.call(req));
            }
            Err(e) => {
                println!("{}", e);
                return Either::Right(ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(json!({
                            "message":"Could not verify json"
                        }))
                        //.header(http::header::LOCATION, "/login")
                        //.finish()
                        .into_body(),
                )));
            }
        }
    }
}
