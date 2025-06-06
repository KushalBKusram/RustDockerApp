use actix_web::{web, App, HttpServer, HttpResponse};

async fn get_health_status() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("Healthy!\n")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(get_health_status))
           // ^ Our new health route points to the get_health_status handler
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}