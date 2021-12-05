use crate::matrix;
use rand;

pub struct ActivationFunction {

}

struct Layer {
	// The values of the neurons in the layer (L)
	neurons: Vec<f32>,
	// The weights from (L-1) to (L)
	weights: Matrix,
	// The biases for the individual neurons
	biases: Vec<f32>,
}

impl Layer {
	pub fn size(&self) -> i32 {
		neurons.len()
	}

	pub fn new(&self, layer_size: i32, last_layer_size: i32) {
		Layer {
			neurons: Vec::with_capacity(layer_size),
			weights: Matrix::new(last_layer, layer.size()),
			biases: Vec::with_capacity(layer_size),
		}
	}

	fn createRandomWeightMatrix(rows: i32, columns: i32) -> Matrix {
		Matrix::new(rows, columns).map() //  Random function
	}

	fn randomval() -> f32

}

pub struct NeuralNetwork {
	input_layer: Vec<f32>,
	hidden_layer: Layer,
	output_layer: Layer,
}

impl NeuralNetwork {
	pub fn new(input_size: i32, hidden_size: Option<i32>, output_size: i32) -> Self {
		let m = matrix::Matrix::new(2, 4);
		NeuralNetwork {
			input_layer: Vec::with_capacity(input_size),
			hidden_layer: Layer::new(hidden_size.unwrap_or(0), input_size),
			output_layer:
		}
	}
}
