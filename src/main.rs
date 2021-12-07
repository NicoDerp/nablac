mod neuralnetwork;
mod matrix;

fn main() {
	let mut neuralnet = neuralnetwork::NeuralNetwork::new(
		10,
		Some(6),
		2,
		Some(Box::new(neuralnetwork::Sigmoid{}))
	);
	println!("{}", neuralnet);
	neuralnet.info();
}
