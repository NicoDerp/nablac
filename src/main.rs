mod neuralnetwork;
mod matrix;

fn main() {
    println!("Hello, world!");
	
	let neuralnet = neuralnetwork::NeuralNetwork::new(10, Some(6), 2);
	println!("{}", neuralnet.getHiddenWeights())
}
