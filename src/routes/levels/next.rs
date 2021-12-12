use afire::{Header, Request, Response, SetCookie};

use crate::level::Level;

pub fn next(req: &Request, levels: Vec<Level>) -> Option<Response> {
    let name = req.path.split_once("/next/")?.1;
    for (i, item) in levels.iter().enumerate() {
        if item.name == name {
            return Some(
                Response::new()
                    .status(308)
                    .header(Header::new(
                        "Location",
                        match &levels.get(i + 1) {
                            Some(i) => format!("/level/{}", i.name),
                            None => "/allDone".to_owned(),
                        },
                    ))
                    .cookie(SetCookie::new("Level", i + 1)),
            );
        }
    }

    Some(Response::new().status(404).text("Level not found :/"))
}
