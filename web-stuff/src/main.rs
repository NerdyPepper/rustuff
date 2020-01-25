use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use serde::{ Deserialize, Serialize };

async fn index1() -> impl Responder {
    println!("recieved request!");
    HttpResponse::Ok().body("all good fam")
}

#[derive(Deserialize, Serialize)]
struct Info {
    id: u32,
    name: String
}

async fn another_route(req: web::Query<Info>) -> impl Responder {
    println!("id: {}, name: {}", req.id, req.name);
    format!("{}{}", req.id, req.name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api").route("/v1", web::to(index1)),
        )
    })
    .bind("127.0.0.1:7878")?
    .run()
    .await
}
