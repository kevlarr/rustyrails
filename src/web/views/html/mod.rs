// Necessary to prevent overflow that can happen during compilation
// when using html! macro inside an `impl Render` block.
// See: https://github.com/lambda-fairy/maud/issues/183
extern crate maud;

mod __layout;
pub use __layout::{Layout, Content};

pub mod root;
pub mod greet;
