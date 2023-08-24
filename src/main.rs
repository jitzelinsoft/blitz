use blitz::run;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    // Activate colored traceback
    color_eyre::install()?;

    // initialize logger
    tracing_subscriber::fmt::init();

    // Run main program
    run().await?;

    Ok(())
}
