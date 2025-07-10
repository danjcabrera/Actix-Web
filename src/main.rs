// The Rust futures and async/await ecosystems are increasingly replacing web actor 
// frameworks like Actix.  
// Actix Web will expose an HTTP server contained within a native executable. 


use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// Request handlers use async functions that accept zero or more parameters. 
// The parameters can be extracted from a request. 
// The return type of the handler must implement the `Responder` trait and can be 
// a `HttpResponse` or any type that implements `Responder`.  

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// Notice that some handlers have routing attributes, like `#[get("/")]` or 
// `#[post("/echo")]` attached directly using the `actix_web` macros.
// These attributes allow you to specify the method and path that the handler 
// responds to.



async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

