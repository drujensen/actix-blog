use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use serde_json::json;

mod config;

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>, config: web::Data<config::Config>) -> impl Responder {
    let default = config.default.clone();
    current(hb, config, default)
}

#[get("/{current}")]
async fn detail(
    hb: web::Data<Handlebars<'_>>,
    config: web::Data<config::Config>,
    path: web::Path<String>,
) -> impl Responder {
    current(hb, config, path.into_inner())
}

fn current(
    hb: web::Data<Handlebars>,
    config: web::Data<config::Config>,
    current: String,
) -> impl Responder {
    let data = json!({
        "title": config.title,
        "description": config.description,
        "posts": config.posts,
        "current": current,
    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("/content/{slug}")]
async fn content(
    config: web::Data<config::Config>,
    hb: web::Data<Handlebars<'_>>,
    path: web::Path<String>,
) -> impl Responder {
    let slug = path.into_inner();
    let post = config.posts.iter().find(|post| post.slug == slug).unwrap();
    let data = json!({
        "slug": slug,
        "title": post.title,
        "author": post.author,
        "date": post.date,
        "body": post.render(),
    });
    let body = hb.render("content", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::new();
    let mut handlebars = Handlebars::new();

    handlebars
        .register_templates_directory(".hbs", "./templates")
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(handlebars.clone()))
            .service(index)
            .service(detail)
            .service(content)
            .service(
                Files::new("/", "./static")
                    .prefer_utf8(true)
                    .use_last_modified(true),
            )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
