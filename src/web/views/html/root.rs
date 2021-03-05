use maud::{html, Markup, Render};
use crate::web::views::html::Content;

pub struct View;

impl Content for View {
    fn title(&self) -> String {
        "Home".into()
    }
}

impl Render for View {
    fn render(&self) -> Markup {
        html! {
            "My home page"
        }
    }
}
