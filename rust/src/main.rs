use tokio;

use {{ package_name }}::Application;
use {{ package_name }}::app_state::AppState;

#[tokio::main]
async fn main() {
    color_eyre::install().expect("Failed to install color_eyre");
    let app_state = AppState {};
    let app = Application::build(app_state, "localhost:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
