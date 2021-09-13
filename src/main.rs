use actix_web::{get, http::StatusCode, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok()
    .body("Hello world!")
    .with_status(StatusCode::OK)
} 

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| App::new().service(hello).service(echo))
    .bind("127.0.0.1:4949")?
    .run()
    .await
}

// #[cfg(test)]
// mod tests {
//     use std::future::Future;
//     use super::*;
//     #[actix_rt::test]
//     async fn test_hello_ok() -> dyn Future<Result<()>, Output = String> {
//         let resp = hello().await.unwrap();
//         assert_eq!(resp.with_status(StatusCode::<OK>));
//         return resp;
//     }
// }
