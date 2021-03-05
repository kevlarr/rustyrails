use maud::{html, Markup, Render};
use crate::web::views::html::Content;

pub struct GreetHtml(pub String);

impl Content for GreetHtml {
    fn title(&self) -> String {
        format!("Greetings to {}", self.0)
    }
}

impl Render for GreetHtml {
    fn render(&self) -> Markup {
        html! {
            h1#greeting {
                "Salutations, "
                (self.0)
                "."
            }
            small#disclaimer {
                "This is a friendly disclaimer to highlight that you, "
                (self.0)
                ", are, in fact, not being greeted cordially."
            }
        }
    }
}
