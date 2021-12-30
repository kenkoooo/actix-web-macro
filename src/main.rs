use actix_web::{get, web, App, HttpServer, Responder};
use actix_web_macro::show_streams;

#[show_streams]
#[get("/{id}/{name}/index.html")]
async fn index1(path: web::Path<(u32, String)>) -> impl Responder {
    let (name, id) = path.into_inner();
    format!("Hello {}! id:{}", name, id)
}
#[show_streams]
#[actix_web::post("/{id}/{name}/index.html")]
async fn index2(path: web::Path<(u32, String)>) -> impl Responder {
    let (name, id) = path.into_inner();
    format!("Hello {}! id:{}", name, id)
}
#[show_streams]
#[::actix_web::put("/{id}/{name}/index.html")]
async fn index3(path: web::Path<(u32, String)>) -> impl Responder {
    let (name, id) = path.into_inner();
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index1))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
