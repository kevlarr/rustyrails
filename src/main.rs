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
        let router = Router::new("/")
            .get("", root)

            // Ultimately I would like to be able to validate that handlers
            // in the scope itself conform to a certain signature with a
            // known scope extractor, eg..
            //
            // .scope::<NameScope>("{name}", |s| s
            //     .get("", greet)
            // )
            //
            // ... but it's over my head so far. See:
            // https://github.com/kevlarr/rustyrails/tree/scoped-handler-sig-validation
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
