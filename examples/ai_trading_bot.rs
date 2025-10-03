use ratb_core::run_prediction;
use ratb_utils::setup_logger;
use anyhow::Result;

fn main() -> Result<()> {
    setup_logger();

    let data_path = "data/market_signals.csv";
    let predictions = run_prediction(data_path)?;

    println!("Predictions:\n{}", predictions);

    Ok(())
}

