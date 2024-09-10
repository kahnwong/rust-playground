use linfa::prelude::*;
use linfa_datasets;
use linfa_trees::DecisionTree;

fn main() {
    // read data
    let dataset = linfa_datasets::iris();

    // set params
    let params = DecisionTree::params();
    // Set the parameters to the desired values
    let params = params.max_depth(Some(5));

    // train
    let tree = params.fit(&dataset).unwrap();

    //test
    let accuracy = tree
        .predict(&dataset)
        .confusion_matrix(&dataset)
        .unwrap()
        .accuracy();

    assert!(accuracy > 0.9);
}
