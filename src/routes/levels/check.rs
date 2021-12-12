use afire::{Request, Response};
use regex::Regex;

use crate::level::Level;

pub fn check(req: &Request, levels: Vec<Level>) -> Option<Response> {
    let name = req.path.split_once("/check/")?.1;
    for i in &levels {
        if i.name == name {
            let req_data = String::from_utf8_lossy(&req.body)
                .split(',')
                .collect::<Vec<&str>>()
                .join(",");

            if Regex::new(&i.correct.join(",")).ok()?.is_match(&req_data) {
                return Some(Response::new().text("CORRECT"));
            }

            return Some(Response::new().text("WRONG"));
        }
    }

    Some(Response::new().status(404).text("Level not found :/"))
}
