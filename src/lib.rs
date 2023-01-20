use pulldown_cmark::{html, Parser};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render_markdown(markdown: String) -> String {
    let mut html_buf = String::new();
    let parser = Parser::new(&markdown[..]);
    html::push_html(&mut html_buf, parser);
    html_buf
}
