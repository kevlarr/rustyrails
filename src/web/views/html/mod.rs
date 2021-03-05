// Necessary to prevent overflow that can happen during compilation
// when using html! macro inside an `impl Render` block.
// See: https://github.com/lambda-fairy/maud/issues/183
extern crate maud;

use actix_web::{
    Error,
    HttpRequest,
    HttpResponse,
    Responder,
};
use futures::future::Ready;
use maud::{html, Markup, Render};

mod root;
pub use root::*;

mod greet;
pub use greet::*;

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
