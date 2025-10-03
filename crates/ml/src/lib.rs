use linfa::prelude::*;
use linfa_trees::DecisionTree;
use ndarray::{Array1, Array2};
use polars::prelude::*;
use log::info;

pub fn train_model(features: &DataFrame, labels: &Series) -> DecisionTree<f64, usize> {
    let feature_matrix = features.to_ndarray::<Float64Type>(IndexOrder::C).unwrap();
    let label_array = labels.u32().unwrap().to_ndarray().unwrap().mapv(|x| x as usize).as_standard_layout().to_owned();

    let dataset = Dataset::new(feature_matrix, label_array);

    info!("Training Decision Tree model...");
    let model = DecisionTree::params().fit(&dataset).unwrap();
    info!("Model training complete.");

    model
}

pub fn predict(model: &DecisionTree<f64, usize>, features: &Array2<f64>) -> Array1<usize> {
    model.predict(features)
}

