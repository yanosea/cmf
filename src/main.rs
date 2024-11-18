mod domain;
mod infrastructure;
mod presentation;
mod usecase;

use presentation::handler::cli_handler;

// main is the entry point of cmf.
fn main() {
    // run the CLI handler
    cli_handler::run();
}
