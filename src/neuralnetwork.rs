use crate::matrix::Matrix;
use std::fmt;

pub trait ActivationFunction {
	fn func(&self, n: f32) -> f32;
	fn dfunc(&self, n: f32) -> f32;
}

pub struct Sigmoid {}
pub struct ReLU {}

impl ActivationFunction for Sigmoid {
	fn func(&self, n: f32) -> f32 {
		1.0 / (1.0 + std::f32::consts::E.powf(-n))
	}

	fn dfunc(&self, n: f32) -> f32 {
		n*(1.0-n)
	}
}

impl ActivationFunction for ReLU {
	fn func(&self, n: f32) -> f32 {
		if n>0.0 {
			n
		} else {
			0.0
		}
	}

	fn dfunc(&self, n: f32) -> f32 {
		if n>0.0 {
			1.0
		} else {
			0.0
		}
	}
}

pub struct Sample {
	data: Vec<f32>,
	label: String,
}

impl Sample {
	pub fn new(data: Vec<f32>, label: String) -> Self {
		Sample {data, label}
	}
}

struct Layer {
	// The values of the neurons in the layer (L)
	neurons: Vec<f32>,
	// The biases for the individual neurons
	biases: Vec<f32>,
	// Size of neurons
	size: i32,
}

impl Layer {
	pub fn size(&self) -> i32 {
		self.size
	}

	pub fn new(layer_size: i32) -> Self {
		Layer {
			neurons: Vec::with_capacity(layer_size.try_into().unwrap()),
			biases: vec![0.0; layer_size.try_into().unwrap()],
			size: layer_size,
		}
	}
}

struct Dataset {
	//info:
}

impl Dataset {
	fn new() -> Self {
		Dataset {}
	}

	fn load() {

	}
}

pub struct NeuralNetwork {
	input_layer: Layer,
	ih_weights: Matrix,
	hidden_layer: Layer,
	ho_weights: Matrix,
	output_layer: Layer,
	//data_set: DataSet,
	activation_function: Box<dyn ActivationFunction>,
}
/**
 * @todo Store every layer in a list
 * @body For layers, there will be n-2 hidden layers, and n-1 weights
 */
impl NeuralNetwork {
	pub fn new(input_size: i32, hidden_size: i32, output_size: i32, func: Option<Box<dyn ActivationFunction>>) -> Self {
		NeuralNetwork {
			input_layer: Layer::new(input_size),
			ih_weights: Matrix::randomized(input_size, hidden_size),
			hidden_layer: Layer::new(hidden_size),
			ho_weights: Matrix::randomized(hidden_size, output_size),
			output_layer: Layer::new(output_size),
			activation_function: func.unwrap_or(Box::new(ReLU{})),
		}
	}

/**
 * @todo Finish architecture of dataset format
 * @body After that I need to implement loading of data.
 */
	pub fn _load_data_set(&self) {

	}

	fn _serealize(&self) {

	}

	fn _deserealize() {

	}

	// Check with the first sample
	pub fn next(&mut self, sample: Sample) {
		if sample.data.len() != self.input_layer.neurons.capacity() {
			panic!("Sample-length ({}) is not the same as the set input capacity ({})", sample.data.len(),self.input_layer.neurons.capacity());
		}
		//self.input_layer.neurons = sample.data.to_vec();
		self.calculate_neurons(sample.data);

		// Cost calculation
	}

	fn calculate_neurons(&mut self, inputs: Vec<f32>) {
		self.input_layer.neurons = inputs.clone();
		self.hidden_layer.neurons = NeuralNetwork::vector_map(
			&NeuralNetwork::vector_addition(
				&Matrix::multiply(
					&self.ih_weights,
					&self.input_layer.neurons
				),
				&self.hidden_layer.biases
			),
		&|x,_| self.activation_function.func(x));

		//self.output_layer.neurons = self.activation_function.func(
		//	NeuralNetwork
		//);
		
	}

	fn vector_map(a: &Vec<f32>, f: &dyn Fn(f32, usize) -> f32) -> Vec<f32> {
		let mut vec = vec![0.0; a.len()];
		for i in 0..a.len() {
			vec[i] = f(vec[i], i);
		}
		vec
	}

	fn vector_addition(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32> {
		if a.len() != b.len() {
			panic!("Vectors has two different lengths a: {}, b: {}", a.len(), b.len());
		}

		let mut c = vec![0.0; a.len()];
		for i in 0..a.len() {
			c[i] = a[i] + b[i];
		}
		c
	}

	pub fn info(&mut self) {
		println!("{}, {}, {}", self.input_layer.size(), self.hidden_layer.size(), self.output_layer.size());
	}
}

impl fmt::Display for NeuralNetwork {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Input-layer size: {}\nHidden-layer size: {}\nOutut-layer size: {}\nActivation function: idk", self.input_layer.size(), self.hidden_layer.size(), self.output_layer.size())
	}
}
