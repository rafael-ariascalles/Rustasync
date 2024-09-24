use tracing::{info, warn, error};
use tracing_subscriber::fmt::format::FmtSpan;


#[tracing::instrument]
async fn long_running_task() -> anyhow::Result<()> {
    println!("Starting long running task");
    Ok(())
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //let subscriber = tracing_subscriber::FmtSubscriber::new();

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT | FmtSpan::CLOSE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("Hello, world!");
    warn!("Warning!");
    error!("Error!");

    long_running_task().await?;
    
    Ok(())
}
