#[tokio::main]
async fn main() {
    let cargo_pkg_name = env!("CARGO_PKG_NAME").replace('-', "_");
    let filter = tracing_subscriber::EnvFilter::new(format!("{}=debug", cargo_pkg_name));
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(false)
        .compact();
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .event_format(format)
        .try_init()
        .unwrap_or_default();

    match run().await {
        Ok(_) => tracing::debug!("🎉 Done"),
        Err(e) => tracing::error!("❌ Failed : {e}"),
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    Ok(())
}
