mod neuralnetwork;
mod matrix;

fn main() {
	let mut neuralnet = neuralnetwork::NeuralNetwork::new(
		10,
		4,
		2,
		None
	);

	let sample = neuralnetwork::Sample::new(vec![0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0], String::from("4"));
	neuralnet.next(sample);

	println!("{}", neuralnet);
}
