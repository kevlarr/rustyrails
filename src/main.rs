use std::io;

use actix_web::{
    web::get,
    App,
    HttpServer,
    Responder,
};

pub mod web {
    pub mod actions {

    }

    pub mod views {
        // Necessary to prevent overflow that can happen during compilation
        // when using html! macro inside an `impl Render` block.
        // See: https://github.com/lambda-fairy/maud/issues/183
        extern crate maud;

        pub mod layout {
            use futures::future::Ready;
            use actix_web::{
                Error,
                HttpRequest,
                HttpResponse,
                Responder,
            };
            
            use maud::{html, Markup, Render};

            pub trait Content: Render {
                fn title(&self) -> String;
            }

            pub struct Layout<C: Content>(pub C);

            impl<C: Content> Render for Layout<C> {
                fn render(&self) -> Markup {
                    html! {
                        DOCTYPE;
                        html {
                            head {
                                title { (self.0.title()) }
                            }
                            body {
                                (self.0.render())
                            }
                        }
                    }
                }
            }

            impl<C: Content> Responder for Layout<C> {
                type Error = Error;
                type Future = Ready<Result<HttpResponse, Error>>;

                fn respond_to(self, req: &HttpRequest) -> Self::Future {
                    self.render().respond_to(req)
                }
            }
        }

        pub mod root {
            use maud::{html, Markup, Render};
            use super::layout::Content;

            pub struct Root {
                pub msg: String,
            }

            impl Content for Root {
                fn title(&self) -> String {
                    "Home".into()
                }
            }

            impl Render for Root {
                fn render(&self) -> Markup {
                    html! {
                        p#wat { (self.msg) }
                    }
                }
            }
        }
    }
}

async fn index() -> impl Responder {
    use web::views::layout::Layout;
    use web::views::root::Root;

    Layout(Root {
        msg: "Hey there".to_string(),
    })
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = || {
        App::new()
            .route("/", get().to(index))
    };
    
    HttpServer::new(app)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
