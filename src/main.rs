mod neuralnetwork;
mod matrix;

fn main() {
	let mut neuralnet = neuralnetwork::NeuralNetwork::new(
		10,
		Some(6),
		2,
		None
	);
	println!("{}", neuralnet.get_hidden_weights());
	println!("{:?}", matrix::Matrix::multiply(neuralnet.get_hidden_weights(), neuralnet.get_inputs()));
}
