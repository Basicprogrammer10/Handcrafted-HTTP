// use std::string::String;

use afire::{ServeStatic, Server};

fn main() {
    let mut server = Server::new("0.0.0.0", 8081);

    // server.middleware(Box::new(|req| {
    //     println!("{}", String::from_utf8(req.raw_data.clone()).unwrap());
    //     None
    // }));

    ServeStatic::new("data/static").attach(&mut server);

    server.start().unwrap();
}
