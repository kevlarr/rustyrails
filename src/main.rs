use std::io;

use actix_web::{
    web::Path,
    App,
    HttpServer,
    Responder,
};

use rustyrails::{
    web::{actions, views::html},
    Router,
};

async fn root() -> impl Responder {
    html::Layout(html::root::View)
}

async fn greet(Path((name,)): Path<(String,)>) -> impl Responder {
    html::Layout(html::greet::View(
        actions::greet::get(name).await
    ))
}


#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = || {
        let routes = Router::new("/")
            .get("", root)
            .get("/{name}", greet)
            .service;

        App::new().service(routes)
    };
    
    HttpServer::new(app)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
