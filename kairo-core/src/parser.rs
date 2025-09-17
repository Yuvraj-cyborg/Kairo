// important stufff
// Parse the markdown into AST and which will be used to form HTML, bheri important need good quality shit

use pulldown_cmark::{Parser, Options, html};

pub fn parse_markdown(md: &str) -> String {
    let parser = Parser::new_ext(md, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

