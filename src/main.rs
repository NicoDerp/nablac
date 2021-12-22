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
<<<<<<< HEAD
		vec![3, 2, 4],
		1.0,
=======
		vec![1, 2, 3, 4],
>>>>>>> d6ac0ca5e73814c550855ae888442fedcfa62a0e
		Some(Box::from(neuralnetwork::Linear{}))
	);

	let samples = vec![
		neuralnetwork::Sample::new(vec![1.0], String::from("3")),
		neuralnetwork::Sample::new(vec![3.0], String::from("4")),
	];
	let dataset = neuralnetwork::Dataset::new(
		samples,
		vec![
			String::from("1"),
			String::from("2"),
			String::from("3"),
			String::from("4"),
		]
	);
	neuralnet.load_dataset(dataset);

	neuralnet.train(1, 1);

	println!("{}\n", neuralnet);
	neuralnet.print_data();

	//let a = matrix::Matrix::from(vec![vec![]]);
}
