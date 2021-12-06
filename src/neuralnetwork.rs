use crate::matrix::Matrix;
use std::fmt;

pub trait ActivationFunction {
	fn func(n: f32) -> f32;
	fn dfunc(n: f32) -> f32;
}

struct Sigmoid {}
struct ReLU {}

impl ActivationFunction for Sigmoid {
	fn func(n: f32) -> f32 {
		1.0 / (1.0 + std::f32::consts::E.powf(-n))
	}

	fn dfunc(n: f32) -> f32 {
		n*(1.0-n)
	}
}

impl ActivationFunction for ReLU {
	fn func(n: f32) -> f32 {
		if n>0.0 {
			n
		} else {
			0.0
		}
	}

	fn dfunc(n: f32) -> f32 {
		if n>0.0 {
			1.0
		} else {
			0.0
		}
	}
	
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
	//data_set: DataSet,
	//activation_function: ActivationFunction,
}

impl NeuralNetwork {
	pub fn new(input_size: i32, hidden_size: Option<i32>, output_size: i32) -> Self {
		NeuralNetwork {
			input_layer: Vec::with_capacity(input_size.try_into().unwrap()),
			hidden_layer: Layer::new(hidden_size.unwrap_or(24), input_size),
			output_layer: Layer::new(output_size, hidden_size.unwrap_or(24)),
			//activation_function: func.unwrap_or(),
		}
	}

/**
 * @todo Finish architecture of dataset format
 * @body After that I need to implement loading of data.
 */
	pub fn loadDataSet() {

	}

	fn _serealize(&self) {

	}

	fn _deserealize() {

	}

	pub fn info(&mut self) {
		self.hidden_layer.neurons.push(5.0);
		self.hidden_layer.neurons[0] = 4.0;
		println!("{}, {}, {}", self.input_layer.len(), self.hidden_layer.size(), self.output_layer.size());
	}
}

impl fmt::Display for NeuralNetwork {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Input-layer size: {}\nHidden-layer size: {}\nOutut-layer size: {}\nActivation function: idk", self.input_layer.len(), self.hidden_layer.size(), self.output_layer.size())
	}
}
