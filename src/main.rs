mod application;
mod domain;
mod infrastructure;
mod interface;

use interface::controller::CliController;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let controller = CliController::new();
    controller.run().await
}
