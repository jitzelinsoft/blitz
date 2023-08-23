use color_eyre::eyre::Result;
use blitz::run;

#[tokio::main]
async fn main() -> Result<()> {
    // Activate colored traceback
    color_eyre::install()?;

    // Run main program
    run().await?;

    Ok(())
}
