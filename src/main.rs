mod neuralnetwork;
mod matrix;

fn main() {
	let mut neuralnet = neuralnetwork::NeuralNetwork::new(10, Some(6), 2);
	println!("{}", neuralnet);
	neuralnet.info();
}
