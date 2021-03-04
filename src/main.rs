use std::io;

use actix_web::{
    web::{get, Path},
    App,
    HttpServer,
    Responder,
};

use rustyrails::web::{views};


async fn root() -> impl Responder {
    use views::html::Layout;
    use views::html::root::Root;

    Layout(Root)
}

async fn greet(Path((name,)): Path<(String,)>) -> impl Responder {
    use views::html::Layout;
    use views::html::greet::Greet;

    Layout(Greet(name))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = || {
        App::new()
            .route("/", get().to(root))
            .route("/{name}", get().to(greet))
    };
    
    HttpServer::new(app)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
