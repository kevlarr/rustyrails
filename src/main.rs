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
    Resource,
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
        let router = Router::new()
            .get("", root)

            //.resource::<NameScope, _>("{name}", |name| name
                //.get("", greet)
            //);

             //TODO scope should be optional?
            .resource("{name}", Resource::<NameScope>::new()
                .get("", greet)
            )
        ;

        App::new().service(router.routes())
    };
    
    HttpServer::new(app)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
