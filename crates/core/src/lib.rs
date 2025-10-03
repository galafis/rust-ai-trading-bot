use ratb_data::load_features_and_labels;
use ratb_ml::{train_model, predict};
use anyhow::Result;
use polars::prelude::*;

pub fn run_prediction(data_path: &str) -> Result<Series> {
    let (features, labels) = load_features_and_labels(data_path)?;
    let model = train_model(&features, &labels);

    let feature_matrix = features.to_ndarray::<Float64Type>(IndexOrder::C)?;
    let predictions = predict(&model, &feature_matrix);

    let predictions_vec: Vec<u32> = predictions.iter().map(|&x| x as u32).collect();
    let predictions_series = Series::new("predictions", predictions_vec);
    Ok(predictions_series)
}

