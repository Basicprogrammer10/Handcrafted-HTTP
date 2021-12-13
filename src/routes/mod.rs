use afire::Server;

mod all_done;
mod grandma;
mod index;
mod levels;

pub fn attach(server: &mut Server) {
    all_done::attach(server);
    grandma::attach(server);
    levels::attach(server);
    index::attach(server);
}
