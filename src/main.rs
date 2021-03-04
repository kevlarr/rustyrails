use std::io;

use actix_web::{
    web::Path,
    App,
    HttpServer,
    Responder,
};

use rustyrails::{
    web::{actions, views},
    Router,
};


async fn root() -> impl Responder {
    use views::html::Layout;
    use views::html::root::Root;

    Layout(Root)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = || {

        let routes = Router::new("/")
            .get("", root)
            .get("/{name}", |path: Path<(String,)>| async {
                use views::html::Layout;
                use views::html::greet::Greet;

                Layout(Greet(
                    actions::greet::get(path).await
                ))
            })
            .service;

        App::new().service(routes)
    };
    
    HttpServer::new(app)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
