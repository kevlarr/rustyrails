use maud::{html, Markup, Render};
use crate::web::{
    scopes::NameScope,
    views::html::Content,
};

pub struct GreetHtml(pub NameScope);

impl Content for GreetHtml {
    fn title(&self) -> String {
        format!("Greetings to {}", self.0.name)
    }
}

impl Render for GreetHtml {
    fn render(&self) -> Markup {
        html! {
            h1#greeting {
                "Salutations, "
                (self.0.name)
                "."
            }
            small#disclaimer {
                "This is a friendly disclaimer to highlight that you, "
                (self.0.name)
                ", are, in fact, not being greeted cordially."
            }
        }
    }
}
