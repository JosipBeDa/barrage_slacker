use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::models::authenticable_users::{AuthenticableUser, AuthData};
use crate::state::app::AppState;
use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage, web};
use futures::future::{ok, Ready};
use crate::error::CustomError;
use base64;

pub struct LoggedGuard;

impl<S> Transform<S, ServiceRequest> for LoggedGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = LoggedGuardMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggedGuardMiddleware { service })
    }
}

pub struct LoggedGuardMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for LoggedGuardMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match is_logged(&req) {
            Ok(auth) => {
                req.extensions_mut().insert(auth);
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Err(e) => {
                println!("Got error: {}", e);
                Box::pin(async move {
                    Ok(ServiceResponse::new(
                        req.into_parts().0,
                        actix_web::HttpResponse::Unauthorized().body(e.to_string()),
                    ))
                })
            }
        }
    }
}

/// Check if the user making the request is logged in
fn is_logged(req: &ServiceRequest) -> Result<AuthenticableUser, CustomError> {
    let header = match &req.headers().get("Authorization") {
        Some(head) => match head.to_str().ok() {
            Some(val) => val.to_string(),
            None => return Err(CustomError::AuthenticationError(String::from("Couldn't parse the header"))),
        },
        None => return Err(CustomError::AuthenticationError(String::from("Couldn't retrieve header"))),
    };
    let mut split = header.split_whitespace();
    let auth_type = split.next();
    if Some("Bearer") == auth_type {
        bearer_auth(split.next().unwrap_or(""))
    } else if Some("Basic") == auth_type {
        basic_auth(
            split.next().unwrap_or(""),
            req,
        )
    } else {
        Err(CustomError::AuthenticationError(String::from("Not valid authentication method")))
    }
}

/// Handle JWT authentication token
fn bearer_auth(data: &str) -> Result<AuthenticableUser, CustomError> {
    match crate::services::jwt::verify(String::from(data)) {
        Ok(user) => Ok(user),
        Err(e) => {
            println!("Got error from jwt: {:?}", e);
            Err(CustomError::AuthenticationError(String::from("Something wrong with the signature")))
        }
    }
}

/// Handle basic auth authentication token
fn basic_auth(data: &str, req: &ServiceRequest) -> Result<AuthenticableUser, CustomError> {
    
    let decoded = base64::decode(data)?;
    let header = String::from(std::str::from_utf8(&decoded)?);
    let mut decoded = header.split(":");
  
    let username = decoded.next().unwrap_or("");
    let password = decoded.next().unwrap_or("");
    let form = AuthData {
        username: String::from(username),
        password: String::from(password)
    };

    // We will try to get app state here and unwrap it, in case the app data does not exist
    // we want to panic, there is no recovery from it missing.
    let state = req.app_data::<web::Data<AppState>>().unwrap();

    let connection = state.db_pool.get().expect("Couldn't get pool conn");

    let (user, _, _) = AuthenticableUser::authenticate(&connection, form)?;

    Ok(user)
  }