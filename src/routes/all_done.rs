use std::fs;

use afire::{Method, Response, Server, SetCookie};

pub fn attach(server: &mut Server) {
    server.route(Method::GET, "/allDone", |_req| {
        Response::new()
            .bytes(fs::read("data/static/allDone/index.html").unwrap())
            .cookie(
                SetCookie::new("Level", "allDone")
                    .path("/")
                    .max_age(365 * 24 * 60 * 60),
            )
    });
}
