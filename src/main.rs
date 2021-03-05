use std::io;

use actix_web::{
    App,
    HttpServer,
    Responder,
};

use rustyrails::{
    web::{
        actions,
        scopes::*,
        views::html::*,
    },
    Router,
};

async fn root() -> impl Responder {
    Layout(RootHtml)
}

async fn greet(scope: NameScope) -> impl Responder {
    Layout(GreetHtml(
        actions::greet::get(scope).await
    ))
}


#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = || {
        let routes = Router::new("/")
            .get("", root)
            .get("{name}", greet)
            .service;

        App::new().service(routes)
    };
    
    HttpServer::new(app)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
