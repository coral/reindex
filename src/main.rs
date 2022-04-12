use anyhow::Result;
use bluos_api_rs::{BluOS, Discovery};
use pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let device = Discovery::discover_one().await?;
    info!("Found BluOS device on: {}", &device.hostname);
    BluOS::new_from_discovered(device)?.update_library().await?;
    info!("Re-indexing the BluOS Library");
    Ok(())
}
