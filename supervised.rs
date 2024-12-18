// Import the Data Structures created in util.rs
use crate::util::{Vector, Matrix, DataPoint};

// Linear Regression Struct
pub struct LinearRegression {
  input_dimensions: usize,
  output_dimensions: usize,
  weights: Vector
}

impl LinearRegression {
  // Constructor for Linear Regression from dimensions
  // Parameters: Input Dim and Output Dim both being usize
  pub fn new_from_dims(input_dimensions: usize, output_dimensions: usize) -> Self {
      LinearRegression {
          input_dimensions,
          output_dimensions,
          weights: Vector::new_from_dims(input_dimensions + 1, 0.0)
      }
  }

  // Constructor for Linear Regression from a DataPoint Data Structure
  // Parameters: A DataPoint Structure
  pub fn new_from_points(points: &DataPoint) -> Self {
   Self {
       input_dimensions: points.get_dimension().0,
       output_dimensions: points.get_dimension().1,
       weights: Vector::new_from_dims(points.get_dimension().0 + 1, 0.0)
   }
  }

  // Training function to fit the linear regression model
  // Parameters: A slice of DataPoint Structures
  pub fn fit(&mut self, training_data: &[DataPoint]) { // training data is given as a slice of datapoints
      // step configs we can change these depending on whatever works best. Also might want to put these in the input parameter
      let learning_rate = 0.001;
      let iterations = 100000;
      // filtering out invalid/none values
      let mut valid_data = Vec::new();
      for data_point in training_data {
          if let Some(y) = &data_point.get_output() {
              if y.get_dimension() == self.output_dimensions {
                  valid_data.push(data_point);
              }
          }
      }
      
      let n = valid_data.len();
      if n == 0 {
          return;
      }

      let mut x_matrix = Matrix::new_from_dims(n, self.input_dimensions + 1, 0.0);
      let mut y_vec = Vector::new_from_dims(n, 0.0);

      for i in 0..n {
          x_matrix[i][0] = 1.0; // intercept
          let input_vec = valid_data[i].get_input();
          for j in 0..self.input_dimensions {
              x_matrix[i][j+1] = input_vec[j];
          }
          y_vec[i] = valid_data[i].get_output().as_ref().unwrap()[0];
      }

      // computing predictions
      for _ in 0..iterations {
          let mut predictions = Vector::new_from_dims(n, 0.0);
          for i in 0..n {
              let mut prediction = 0.0;
              for j in 0..(self.input_dimensions + 1) {
                  prediction += x_matrix[i][j] * self.weights[j];
              }
              predictions[i] = prediction;
          }
          // computing errors (predictions - correct y value)
          let mut errors = Vector::new_from_dims(n, 0.0);
          for i in 0..n {
              errors[i] = predictions[i] - y_vec[i];
          }
          // gradient = (1/N)*Σ(errors[i]*X[i][j]
          let mut gradient = Vector::new_from_dims(self.input_dimensions + 1, 0.0);
          for i in 0..(self.input_dimensions + 1) {
              let mut sum = 0.0;
              for j in 0..n {
                  sum += errors[j] * x_matrix[j][i];
              }
              gradient[i] = (1.0 / (n as f64) * sum);
          }
          // update the weights
          for i in 0..(self.input_dimensions + 1) {
              self.weights[i] -= learning_rate * gradient[i];
          }
      }
  }

  // Evaluating the fitted model given a slice of DataPoint as a testing data
  // Parameters: A slice of DataPoint Structures
  pub fn eval(&self, points: &[DataPoint]) -> f64 { 
       // Finding the Mean Squared Error from the given points and the plotted Linear Regression Function
       let mut error:f64 = 0.0;

       for i in 0..self.input_dimensions {
           let mut prediction = 0.0;
           for j in 0..self.input_dimensions {
               prediction += points[i].get_input()[j] * self.weights[j + 1];
           }
           prediction += self.weights[self.input_dimensions - 1]; // Constant Term
           let diff = prediction - points[i].get_output().as_ref().unwrap()[0];
           error += diff.abs() * diff.abs();
       }

       error
  }

  // Get Function for retrieving coefficients
  pub fn get_coefficients(&self) -> Vec<f64> {
      self.weights.as_vec()[1..].to_vec()
  }

  // Get Function for retrieving intercepts
  pub fn get_intercept(&self) -> f64 {
      self.weights[0]
  }

  // Get Function for retrieving dimensions
  pub fn get_dimensions(&self) -> (usize, usize) {
       (self.input_dimensions, self.output_dimensions)
  }
}
