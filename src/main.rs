mod domain;
mod infrastructure;
mod presentation;
mod usecase;

use presentation::handler::cli_handler;

fn main() {
    cli_handler::run();
}
