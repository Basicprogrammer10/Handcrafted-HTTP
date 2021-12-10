// use std::string::String;

use afire::{ServeStatic, Server};

fn main() {
    let mut server = Server::new("localhost", 8082);

    // server.middleware(Box::new(|req| {
    //     println!("{}", String::from_utf8(req.raw_data.clone()).unwrap());
    //     None
    // }));

    // Serve Static
    ServeStatic::new("data/static").attach(&mut server);

    server.start().unwrap();
}
