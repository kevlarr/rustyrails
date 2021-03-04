use maud::{html, Markup, Render};
use crate::web::views::html::Content;

pub struct Root;

impl Content for Root {
    fn title(&self) -> String {
        "Home".into()
    }
}

impl Render for Root {
    fn render(&self) -> Markup {
        html! {
            "My home page"
        }
    }
}
