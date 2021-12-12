#[macro_use]
extern crate lazy_static;
use afire::{Logger, ServeStatic, Server};

mod level;
mod routes;

const LEVEL_PATH: &str = "./data/levels";

pub static mut LEVELS: Vec<level::Level> = Vec::new();

fn main() {
    let mut server = Server::new("localhost", 8080);

    // Serve Static
    ServeStatic::new("data/static").attach(&mut server);

    // Load Levels
    let levels = level::Level::load_all(LEVEL_PATH).unwrap();
    unsafe { LEVELS = levels.clone() }

    // Serve Routes
    routes::attach(&mut server);

    // Start Logger
    Logger::new().attach(&mut server);

    println!("Serveing: {}:{}", server.ip_string(), server.port);

    server.start().unwrap();
}
