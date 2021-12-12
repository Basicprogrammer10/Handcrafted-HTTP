use afire::Server;

mod index;
mod levels;

pub fn attach(server: &mut Server) {
    levels::attach(server);
    index::attach(server);
}
