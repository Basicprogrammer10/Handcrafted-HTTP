use afire::Server;

mod grandma;
mod index;
mod levels;

pub fn attach(server: &mut Server) {
    grandma::attach(server);
    levels::attach(server);
    index::attach(server);
}
