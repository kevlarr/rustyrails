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
    Layout(GreetHtml(scope))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = || {
        let router = Router::new("/")
            .get("", root)

            .scope("{name}", |name| name
                .get("", greet)
            );

        App::new().service(router.service())
    };
    
    HttpServer::new(app)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
