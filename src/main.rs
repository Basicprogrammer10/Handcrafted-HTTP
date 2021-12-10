// use std::string::String;

use afire::{ServeStatic, Server};

mod level;

const LEVEL_PATH: &str = "./data/levels";

fn main() {
    let mut server = Server::new("localhost", 8080);

    // server.middleware(Box::new(|req| {
    //     println!("{}", String::from_utf8(req.raw_data.clone()).unwrap());
    //     None
    // }));

    // Load Levels
    let levels = level::Level::load_all(LEVEL_PATH).unwrap();
    level::Level::attach(&mut server, levels);

    ServeStatic::new("data/static").attach(&mut server);

    println!("Serveing: {}:{}", server.ip_string(), server.port);

    server.start().unwrap();
}
