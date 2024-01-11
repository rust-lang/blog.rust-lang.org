use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    blog::main()?;

    let footer = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    warpy::server::run(
        format!("{}/../site", env!("CARGO_MANIFEST_DIR")),
        [0, 0, 0, 0],
        footer,
        Some(8000),
        false,
    )
    .await?;
    Ok(())
}
