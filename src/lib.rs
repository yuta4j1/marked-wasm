extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use pulldown_cmark::{Parser, Options, html};

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n"
            , parse("Hello world, this is a ~~complicated~~ *very simple* example."))
    }
}
