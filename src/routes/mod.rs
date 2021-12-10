use afire::Server;

mod index;

pub fn attach(server: &mut Server) {
    index::attach(server);
}
