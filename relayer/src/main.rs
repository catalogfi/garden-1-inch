mod orderbook;
mod primitives;
mod server;
mod settings;
use server::Server;

use crate::settings::Settings;

const CONFIG_FILE: &str = "Settings.toml";

#[tokio::main]
async fn main() {
    let settings = Settings::from_toml(CONFIG_FILE);

    let _ = tracing_subscriber::fmt()
        .with_env_filter("relayer=info,sqlx=warn")
        .try_init();

    let server = Server::new(settings.port, &settings.db_url)
        .await
        .expect("Failed to initialize server");
    server.run().await;
}
