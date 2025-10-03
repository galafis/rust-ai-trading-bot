use polars::prelude::*;
use anyhow::Result;

pub fn load_features_and_labels(path: &str) -> Result<(DataFrame, Series)> {
    let df = CsvReader::from_path(path)?.finish()?;
    let features = df.select(["feature1", "feature2"])?;
    let labels = df.column("label")?.cast(&DataType::UInt32)?.clone();
    Ok((features, labels))
}

