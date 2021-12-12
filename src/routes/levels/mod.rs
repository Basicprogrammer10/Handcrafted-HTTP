use afire::{Request, Response, Server};

use crate::LEVELS;

mod check;
mod level;
mod next;

lazy_static! {
    static ref ERR_RES: Response = Response::new()
        .status(500)
        .text("sorwy :|\nInternal Server Error.");
}

pub fn attach(server: &mut Server) {
    server.middleware(Box::new(|req| run(req)));
}

fn run(req: &Request) -> Option<Response> {
    let levels = unsafe { LEVELS.clone() };

    if req.path.starts_with("/level/") {
        return Some(level::level(req, levels).unwrap_or(ERR_RES.clone()));
    }

    if req.path.starts_with("/check/") {
        return Some(check::check(req, levels).unwrap_or(ERR_RES.clone()));
    }

    if req.path.starts_with("/next/") {
        return Some(next::next(req, levels).unwrap_or(ERR_RES.clone()));
    }

    None
}
