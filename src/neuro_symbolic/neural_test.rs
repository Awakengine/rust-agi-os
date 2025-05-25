#[cfg(test)]
mod tests {
    use super::*;
    use crate::neuro_symbolic::neural::{NeuralModel, Layer, LinearLayer, Tensor, NeuralError};

    #[test]
    fn test_tensor_creation() {
        let tensor = Tensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        
        assert_eq!(tensor.shape, vec![2, 3]);
        assert_eq!(tensor.data, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_tensor_reshape() {
        let mut tensor = Tensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        tensor.reshape(vec![3, 2]).unwrap();
        
        assert_eq!(tensor.shape, vec![3, 2]);
        assert_eq!(tensor.data, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_tensor_reshape_error() {
        let mut tensor = Tensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let result = tensor.reshape(vec![4, 2]);
        
        assert!(result.is_err());
        if let Err(NeuralError::ShapeError(msg)) = result {
            assert!(msg.contains("incompatible"));
        } else {
            panic!("Expected ShapeError");
        }
    }

    #[test]
    fn test_tensor_get() {
        let tensor = Tensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        
        assert_eq!(tensor.get(&[0, 0]).unwrap(), 1.0);
        assert_eq!(tensor.get(&[0, 1]).unwrap(), 2.0);
        assert_eq!(tensor.get(&[0, 2]).unwrap(), 3.0);
        assert_eq!(tensor.get(&[1, 0]).unwrap(), 4.0);
        assert_eq!(tensor.get(&[1, 1]).unwrap(), 5.0);
        assert_eq!(tensor.get(&[1, 2]).unwrap(), 6.0);
    }

    #[test]
    fn test_tensor_get_error() {
        let tensor = Tensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        
        assert!(tensor.get(&[2, 0]).is_err());
        assert!(tensor.get(&[0, 3]).is_err());
        assert!(tensor.get(&[0]).is_err());
    }

    #[test]
    fn test_tensor_set() {
        let mut tensor = Tensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        
        tensor.set(&[0, 0], 10.0).unwrap();
        tensor.set(&[1, 2], 20.0).unwrap();
        
        assert_eq!(tensor.get(&[0, 0]).unwrap(), 10.0);
        assert_eq!(tensor.get(&[1, 2]).unwrap(), 20.0);
    }

    #[test]
    fn test_tensor_set_error() {
        let mut tensor = Tensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        
        assert!(tensor.set(&[2, 0], 10.0).is_err());
        assert!(tensor.set(&[0, 3], 10.0).is_err());
        assert!(tensor.set(&[0], 10.0).is_err());
    }

    #[test]
    fn test_tensor_add() {
        let tensor1 = Tensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let tensor2 = Tensor::new(vec![2, 3], vec![6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
        
        let result = tensor1.add(&tensor2).unwrap();
        
        assert_eq!(result.shape, vec![2, 3]);
        assert_eq!(result.data, vec![7.0, 7.0, 7.0, 7.0, 7.0, 7.0]);
    }

    #[test]
    fn test_tensor_add_error() {
        let tensor1 = Tensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let tensor2 = Tensor::new(vec![3, 2], vec![6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
        
        assert!(tensor1.add(&tensor2).is_err());
    }

    #[test]
    fn test_tensor_multiply() {
        let tensor1 = Tensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let tensor2 = Tensor::new(vec![3, 2], vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
        
        let result = tensor1.multiply(&tensor2).unwrap();
        
        assert_eq!(result.shape, vec![2, 2]);
        // (1*7 + 2*9 + 3*11) = 58, (1*8 + 2*10 + 3*12) = 64
        // (4*7 + 5*9 + 6*11) = 139, (4*8 + 5*10 + 6*12) = 154
        assert_eq!(result.data, vec![58.0, 64.0, 139.0, 154.0]);
    }

    #[test]
    fn test_tensor_multiply_error() {
        let tensor1 = Tensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let tensor2 = Tensor::new(vec![2, 2], vec![7.0, 8.0, 9.0, 10.0]);
        
        assert!(tensor1.multiply(&tensor2).is_err());
    }

    #[test]
    fn test_linear_layer() {
        let mut layer = LinearLayer::new(2, 1);
        
        // Set weights and bias for deterministic test
        layer.weights = Tensor::new(vec![2, 1], vec![0.5, 0.5]);
        layer.bias = Tensor::new(vec![1], vec![0.0]);
        
        let input = Tensor::new(vec![1, 2], vec![2.0, 2.0]);
        let output = layer.forward(&input).unwrap();
        
        assert_eq!(output.shape, vec![1, 1]);
        assert_eq!(output.data[0], 2.0); // 2*0.5 + 2*0.5 + 0 = 2.0
    }

    #[test]
    fn test_neural_error() {
        let shape_error = NeuralError::ShapeError("Invalid shape".to_string());
        let param_error = NeuralError::ParameterError("Invalid parameter".to_string());
        
        assert_eq!(
            format!("{}", shape_error),
            "Shape error: Invalid shape"
        );
        assert_eq!(
            format!("{}", param_error),
            "Parameter error: Invalid parameter"
        );
    }

    #[test]
    fn test_module_functions() {
        // Test initialization functions
        assert!(init().is_ok());
        assert!(start().is_ok());
        assert!(stop().is_ok());
    }
}
