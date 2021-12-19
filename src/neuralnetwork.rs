use crate::matrix::Matrix;
use std::fmt;

pub trait ActivationFunction {
	fn func(&self, n: f32) -> f32;
	fn dfunc(&self, n: f32) -> f32;
	fn name(&self) -> &str;
}

pub struct Sigmoid {}
pub struct ReLU {}
pub struct Tanh {}
pub struct Linear {}

impl ActivationFunction for Sigmoid {
	fn func(&self, n: f32) -> f32 {
		1.0 / (1.0 + std::f32::consts::E.powf(-n))
	}

	fn dfunc(&self, n: f32) -> f32 {
		n*(1.0-n)
	}


	fn name(&self) -> &str {
		"Sigmoid"
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

	fn name(&self) -> &str {
		"ReLU"
	}
}


impl ActivationFunction for Linear {
    fn func(&self, n: f32) -> f32 {
    	n
    }

    fn dfunc(&self, n: f32) -> f32 {
    	1.0
    }
    
    fn name(&self) -> &str {
    	"Linear"
    }
}

impl ActivationFunction for Tanh {
	fn func(&self, n: f32) -> f32 {
		(std::f32::consts::E.powf(n) - std::f32::consts::E.powf(-n)) / (std::f32::consts::E.powf(n) + std::f32::consts::E.powf(-n))
	}

	fn dfunc(&self, n: f32) -> f32 {
		4.0 / (std::f32::consts::E.powf(-n) + std::f32::consts::E.powf(n)).powf(2.0)
	}

	fn name(&self) -> &str {
		"Tanh"
	}
}

pub struct Sample {
	data: Vec<f32>,
	label: String,
}

impl Clone for Sample {
	fn clone(&self) -> Sample {
		let mut vec=vec![];
		for i in 0..self.data.len() {
			vec.push(self.data[i]);
		}
		Sample {
			data: vec,
			label: self.label.clone()
		}
	}
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
			neurons: vec![0.0; layer_size.try_into().unwrap()],
			biases: vec![0.0; layer_size.try_into().unwrap()],
			size: layer_size,
		}
	}
}

pub struct Dataset {
	samples: Vec<Sample>,
	labels: Vec<String>,
}

impl Dataset {
	pub fn new(samples: Vec<Sample>, labels: Vec<String>) -> Self {
		Dataset {
			samples,
			labels,
		}
	}

	pub fn _load_info_file(path: String) {

	}
}

pub struct NeuralNetwork {
	layers: Vec<Layer>,
	weights: Vec<Matrix>,
	//input_layer: Layer,
	//ih_weights: Matrix,
	//hidden_layer: Layer,
	//ho_weights: Matrix,
	//output_layer: Layer,
	dataset: Option<Dataset>,
	activation_function: Box<dyn ActivationFunction>,
}
/**
 * @todo Store every layer in a list
 * @body For layers, there will be n-2 hidden layers, and n-1 weights
 */

/**
 * @todo Should also implement an optimalization method
 * @body Many different types. https://iamtrask.github.io/2015/07/27/python-network-part2/
 */
impl NeuralNetwork {
	pub fn new(layer_neurons: Vec<i32>, func: Option<Box<dyn ActivationFunction>>) -> Self {
		let mut l = Vec::new();
		for i in 0..layer_neurons.len() {
			println!("Initialized {} neurons", layer_neurons[i]);
			l.push(Layer::new(layer_neurons[i]));
		}

		let mut w = Vec::new();
		for i in 1..layer_neurons.len() {
			// Randomized
			println!("Initializing weight {}, with connections from {} to {}", i, layer_neurons[i-1], layer_neurons[i]);
			w.push(Matrix::new(layer_neurons[i-1].try_into().unwrap(), layer_neurons[i].try_into().unwrap()));
		}

		NeuralNetwork {
			layers: l,
			weights: w,
			dataset:None,
			activation_function: func.unwrap_or(Box::new(ReLU{})),
		}
	}

/**
 * @todo Finish architecture of dataset format
 * @body After that I need to implement loading of data.
 */
	pub fn load_dataset(&mut self, ds: Dataset) {
		// If labels have a different length than ouput layer length
		if ds.labels.len() != self.layers[self.layers.len()-1].neurons.len() {
			panic!("Tried loading dataset with len {}, but the ouput layer has a different len {}.", ds.labels.len(), self.layers[self.layers.len()-1].neurons.len());
		}

		self.dataset = Some(ds);
	}

	fn _serealize(&self) {

	}

	fn _deserealize() {

	}

	pub fn print_data(&self) {
		println!("Input layer:\n{:?}\n", self.layers[0].neurons);
		for i in 1..self.layers.len() {
			println!("Weights:\n{}\n\nNeurons layer {}:\n{:?}\n", self.weights[i-1], i, self.layers[i].neurons)
		}
	}

	/**
	 * @todo Implement shuffle for samples each epoch
	 */
	// Trains for the whole dataset
	pub fn train(&mut self, epochs: usize, batch_size: usize) {
		if self.dataset.is_none() {
			panic!("train function was called, but no dataset is loaded! Use the load_dataset function to load.");
		}

		//let mut samples_len = 0;
		//{
		//	let mut dataset = &self.dataset.unwrap();
		//	samples_len = dataset.samples.len();
		//}
		for epoch in 0..epochs {
			for i in 0..self.dataset.as_ref().unwrap().samples.len() {
				if self.dataset.as_ref().unwrap().samples[i].data.len() != self.layers[0].neurons.capacity() {
					panic!("Sample-length ({}) is not the same as the set input capacity ({})", self.dataset.as_ref().unwrap().samples[i].data.len(),self.layers[0].neurons.capacity());
				}

				// Feed forward sample
				self.layers[0].neurons = self.dataset.as_ref().unwrap().samples[i].data.clone();
				self.feed_forward();

				println!("i%batch {}", i % batch_size);
				if i % batch_size == 0 {
					println!("All samples in batch complete");
					// Create a 1 hot vector from labels
					let mut correct_labels = vec![];
					for j in 0..self.dataset.as_ref().unwrap().labels.len() {
						if self.dataset.as_ref().unwrap().samples[i].label == self.dataset.as_ref().unwrap().labels[j] {
							correct_labels.push(1.0);
						} else {
							correct_labels.push(0.0);
						}
					}
					
					// Cost calculation
					let C = NeuralNetwork::vector_map(
						// Output layer - labels
						&NeuralNetwork::vector_subtraction(&self.layers[self.layers.len()-1].neurons, &correct_labels),
						// Derivative, Sigmoid's dfunc = x(1-x)
						&|x,_| self.activation_function.dfunc(x));

					println!("{:?}", C);
		
					// Gradient calculation

					// Updating weights
				}
			}
		}
	}

	fn feed_forward(&mut self) {//, sample: Sample) {
		//self.layers[0].neurons = sample.data.clone();
		for i in 1..self.layers.len() {
			println!("\nStarted on feed-forwarding layer {}", i);
			println!("Layer {} has {:?} neurons", i-1, self.layers[i-1].neurons);
			// Maybe transpose
			let mut v = Matrix::multiply(&self.weights[i-1].transpose(), &self.layers[i-1].neurons);
			v = NeuralNetwork::vector_addition(&v, &self.layers[i-1].biases);
			v = NeuralNetwork::vector_map(&v, &|x,_| self.activation_function.func(x));
			self.layers[i].neurons = v;
		}
	}

	pub fn vector_map(a: &Vec<f32>, f: &dyn Fn(f32, usize) -> f32) -> Vec<f32> {
		let mut vec = a.to_vec();
		for i in 0..vec.len() {
			vec[i] = f(vec[i], i);
		}
		vec
	}

	pub fn vector_addition(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32> {
		if a.len() != b.len() {
			panic!("Vectors have two different lengths a: {}, b: {}", a.len(), b.len());
		}

		let mut vec = vec![0.0; a.len()];
		for i in 0..a.len() {
			vec[i] = a[i] + b[i];
		}
		vec
	}

	pub fn vector_subtraction(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32> {
		if a.len() != b.len() {
			panic!("Vectors have two different lengths a: {}, b: {}", a.len(), b.len());
		}

		let mut vec = vec![0.0; a.len()];
		for i in 0..a.len() {
			vec[i] = a[i] - b[i];
		}
		vec
	}
}

impl fmt::Display for NeuralNetwork {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Input-layer size: {}\nHidden-layers: {}\nOutut-layer size: {}\nActivation function: {}", self.layers[0].size(), self.layers.len()-2, self.layers[self.layers.len()-1].size(), self.activation_function.name())
	}
}
