mod neuralnetwork;
mod matrix;

/**
 * @todo Create a NeuralNetwork wrapper
 * @body For easier use and adding functionality at the same time.
 * @body The entire library will be called NablaC. Will implement loading of datasets.
 * @body Should be able to use custom and prebuilt datasets like the mnist dataset.
 */
fn main() {
	let mut neuralnet = neuralnetwork::NeuralNetwork::new(
		vec![3, 2, 2],
		Some(Box::from(neuralnetwork::Linear{}))
	);

	let sample = neuralnetwork::Sample::new(vec![1.0, 1.0, 1.0], String::from("2"));
	let dataset = neuralnetwork::Dataset::new(
		vec![sample],
		vec![String::from("1"), String::from("2")]
	);
	neuralnet.load_dataset(dataset);

	neuralnet.train(1, 1);

	println!("{}\n", neuralnet);
	neuralnet.print_data();

	//let a = matrix::Matrix::from(vec![vec![]]);
}
