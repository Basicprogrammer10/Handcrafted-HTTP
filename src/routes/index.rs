use std::fs;

use afire::{Header, Method, Response, Server};

use crate::LEVELS;

pub fn attach(server: &mut Server) {
    server.route(Method::GET, "/", |req| {
        if req.query.get("base").is_some() {
            return Response::new().bytes(fs::read("data/static/index.html").unwrap());
        }

        match req.cookies.iter().find(|x| x.name == "Level") {
            Some(i) => match verify_level(&i.value) {
                true => Response::new()
                    .status(308)
                    .header(Header::new("Location", format!("/level/{}", i.value))),
                false => {
                    if i.value == "allDone" {
                        return Response::new()
                            .status(308)
                            .header(Header::new("Location", "/allDone"));
                    }
                    Response::new().bytes(fs::read("data/static/index.html").unwrap())
                }
            },
            None => Response::new().bytes(fs::read("data/static/index.html").unwrap()),
        }
    });
}

fn verify_level(level: &str) -> bool {
    let levels = unsafe { &LEVELS };
    levels.iter().find(|x| x.name == level).is_some()
}
