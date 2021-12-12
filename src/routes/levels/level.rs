use std::fs;

use afire::{Request, Response};

use crate::level::Level;

pub fn level(req: &Request, levels: Vec<Level>) -> Option<Response> {
    let name = req.path.split_once("/level/")?.1;
    for i in levels {
        if i.name == name {
            let mut options = String::new();

            for j in &i.options {
                options.push_str(r#"<div class="drag">"#);
                options.push_str(&j);
                options.push_str(r#"</div>"#)
            }

            let base = fs::read_to_string("data/template/level.html")
                .ok()?
                .replace(
                    "{{README}}",
                    &markdown::to_html(&i.readme)
                        .replace("&lt;br&gt;", "⏎<br>")
                        .replace("&lt;br/&gt;", "<br>"),
                )
                .replace("{{LEVEL}}", &i.name)
                .replace("{{OPTIONS}}", &options);
            return Some(Response::new().text(base));
        }
    }

    Some(Response::new().status(404).text("Level not found :/"))
}
