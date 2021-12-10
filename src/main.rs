use afire::{ServeStatic, Server};

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
    level::Level::attach(&mut server, levels);

    // Serve Other Routes
    routes::attach(&mut server);

    println!("Serveing: {}:{}", server.ip_string(), server.port);

    server.start().unwrap();
}
