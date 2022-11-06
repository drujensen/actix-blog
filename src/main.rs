use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod config;

#[get("/")]
async fn index(config: web::Data<config::Config>) -> impl Responder {
    current(config.default.clone())
}

#[get("/{current}")]
async fn detail(path: web::Path<String>) -> impl Responder {
    current(path.into_inner())
}

fn current(current: String) -> impl Responder {
    HttpResponse::Ok().body(current)
}

#[get("/content/{slug}")]
async fn content(config: web::Data<config::Config>, path: web::Path<String>) -> impl Responder {
    let slug = path.into_inner();
    let post = config.posts.iter().find(|post| post.slug == slug).unwrap();
    post.render()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(config::Config::new()))
            .service(index)
            .service(detail)
            .service(content)
            .service(
                Files::new("/", "./static")
                    .prefer_utf8(true)
                    .use_last_modified(true),
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
