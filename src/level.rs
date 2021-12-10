use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;

use afire::{Method, Response, Server};

#[derive(Debug, Clone)]
pub struct Level {
    pub name: String,
    pub readme: String,
    pub options: Vec<String>,
    pub correct: Vec<String>,
}

impl Level {
    pub fn load_all(path: &str) -> Option<Vec<Level>> {
        let base_path = PathBuf::from(path);
        let mut out = Vec::new();

        let folders = fs::read_dir(&base_path)
            .ok()?
            .map(|x| x.unwrap())
            .filter(|x| x.path().is_dir())
            .collect::<Vec<DirEntry>>();

        for i in folders {
            let name = i.file_name().to_str()?.to_owned();
            let readme = fs::read_to_string(i.path().join("readme.md")).ok()?;
            let options = fs::read_to_string(i.path().join("options.txt"))
                .ok()?
                .lines()
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            let correct = fs::read_to_string(i.path().join("correct.txt"))
                .ok()?
                .lines()
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();

            out.push(Level {
                name,
                readme,
                options,
                correct,
            });
        }

        Some(out)
    }

    pub fn attach(server: &mut Server, levels: Vec<Level>) {
        let level = levels.clone();

        // Get Level Page
        server.middleware(Box::new(move |req| {
            if req.method != Method::GET || !req.path.starts_with("/level/") {
                return None;
            }

            let name = req.path.split_once("/level/").unwrap().1;
            for i in &level {
                if i.name == name {
                    let mut options = String::new();

                    for j in &i.options {
                        options.push_str(r#"<div class="drag keep">"#);
                        options.push_str(&j);
                        options.push_str(r#"</div>"#)
                    }

                    let base = fs::read_to_string("data/template/level.html")
                        .unwrap()
                        .replace(
                            "{{README}}",
                            &markdown::to_html(&i.readme).replace("&lt;br&gt;", "⏎<br>"),
                        )
                        .replace("{{OPTIONS}}", &options);
                    return Some(Response::new().text(base));
                }
            }

            Some(Response::new().status(404).text("Level not found :/"))
        }));

        // Check Solution
        server.middleware(Box::new(move |req| {
            if req.method != Method::POST || !req.path.starts_with("/check/") {
                return None;
            }

            let name = req.path.split_once("/check/").unwrap().1;
            for i in &levels {
                if i.name == name {
                    dbg!(i.correct.join(","));
                    if String::from_utf8_lossy(&req.body) != i.correct.join(",") {
                        return Some(Response::new().text("WRONG"));
                    }

                    return Some(Response::new().text("CORRECT"));
                }
            }

            Some(Response::new().status(404).text("Level not found :/"))
        }));
    }
}
