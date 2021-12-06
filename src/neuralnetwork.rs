use crate::matrix::Matrix;

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
	pub fn size(&self) -> usize {
		self.neurons.len()
	}

	pub fn new(layer_size: i32, last_layer_size: i32) -> Self {
		Layer {
			neurons: Vec::with_capacity(layer_size.try_into().unwrap()),
			weights: Matrix::randomized(last_layer_size.try_into().unwrap(), layer_size.try_into().unwrap()),
			biases: Vec::with_capacity(layer_size.try_into().unwrap()),
		}
	}
}

pub struct NeuralNetwork {
	input_layer: Vec<f32>,
	hidden_layer: Layer,
	output_layer: Layer,
}

impl NeuralNetwork {
	pub fn new(input_size: i32, hidden_size: Option<i32>, output_size: i32) -> Self {
		NeuralNetwork {
			input_layer: Vec::with_capacity(input_size.try_into().unwrap()),
			hidden_layer: Layer::new(hidden_size.unwrap_or(24), input_size),
			output_layer: Layer::new(output_size, hidden_size.unwrap_or(24)),
		}
	}

	pub fn getHiddenWeights(&self) -> &Matrix {
		&self.hidden_layer.weights
	}
}
