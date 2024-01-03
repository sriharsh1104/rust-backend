// use actix_service::Service;
// use actix_web::{dev, Error, FromRequest, HttpRequest, HttpMessage, Result};
// use futures::future::{ready, Ready};
// use std::pin::Pin;
// use std::task::{Context, Poll};

// pub struct JwtAuth;

// impl<S, B> Middleware<S, B> for JwtAuth
// where
//     S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     B: MessageBody,
// {
//     type Request = ServiceRequest;
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = Ready<Result<Self::Response, Self::Error>>;

//     fn call(&self, req: ServiceRequest, srv: S) -> Self::Future {
//         let token_result = extract_jwt_token(&req);
        
//         match token_result {
//             Ok(token) => {
//                 match verify_jwt_token(&token) {
//                     Ok(token_data) => {
//                         // You can now access token_data.claims to get user claims
//                         let user_id = token_data.claims.sub;
//                         // Add your authorization logic here
//                         ready(Ok(req))
//                     }
//                     Err(_) => ready(Err(error_response("Invalid token")))
//                 }
//             }
//             Err(_) => ready(Err(error_response("Token not provided")))
//         }
//     }
// }

// fn error_response(message: &str) -> ServiceResponse<Body> {
//     HttpResponse::Unauthorized()
//         .json(ApiResponse {
//             message: message.to_string(),
//             token: None,
//         })
//         .into()
// }

// fn extract_jwt_token(req: &ServiceRequest) -> Result<String, Error> {
//     if let Some(header_value) = req.headers().get("Authorization") {
//         if let Ok(auth_header) = header_value.to_str() {
//             if auth_header.starts_with("Bearer ") {
//                 let token = auth_header[7..].to_string();
//                 return Ok(token);
//             }
//         }
//     }

//     Err(error_response("Invalid Authorization header"))
// }
