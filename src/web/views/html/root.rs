use maud::{html, Markup, Render};
use crate::web::views::html::Content;

pub struct RootHtml;

impl Content for RootHtml {
    fn title(&self) -> String {
        "Home".into()
    }
}

impl Render for RootHtml {
    fn render(&self) -> Markup {
        html! {
            "My home page"
        }
    }
}
