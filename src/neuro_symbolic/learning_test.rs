#[cfg(test)]
mod tests {
    use super::*;
    use crate::neuro_symbolic::learning::{Learner, NeuralLearner, SymbolicLearner, LearningError};
    use crate::neuro_symbolic::neural::{NeuralModel, Tensor};
    use crate::neuro_symbolic::symbolic::{KnowledgeBase, SimpleKnowledgeBase};
    use std::sync::{Arc, Mutex};

    // Mock neural model for testing
    #[derive(Debug)]
    struct MockNeuralModel {
        trained: bool,
        reset_called: bool,
    }

    impl MockNeuralModel {
        fn new() -> Self {
            Self {
                trained: false,
                reset_called: false,
            }
        }
    }

    impl NeuralModel for MockNeuralModel {
        fn forward(&self, input: &Tensor) -> Result<Tensor, crate::neuro_symbolic::neural::NeuralError> {
            Ok(Tensor::new(vec![1, 1], vec![1.0]))
        }

        fn backward(&mut self, gradient: &Tensor) -> Result<Tensor, crate::neuro_symbolic::neural::NeuralError> {
            Ok(Tensor::new(vec![1, 1], vec![1.0]))
        }

        fn update(&mut self, learning_rate: f32) -> Result<(), crate::neuro_symbolic::neural::NeuralError> {
            Ok(())
        }

        fn train(&mut self, data: &Tensor) -> Result<(), crate::neuro_symbolic::neural::NeuralError> {
            self.trained = true;
            Ok(())
        }

        fn save(&self, path: &str) -> Result<(), crate::neuro_symbolic::neural::NeuralError> {
            Ok(())
        }

        fn load(&mut self, path: &str) -> Result<(), crate::neuro_symbolic::neural::NeuralError> {
            Ok(())
        }

        fn reset(&mut self) -> Result<(), crate::neuro_symbolic::neural::NeuralError> {
            self.reset_called = true;
            Ok(())
        }
    }

    #[test]
    fn test_neural_learner() {
        let model = Box::new(MockNeuralModel::new());
        let mut learner = NeuralLearner::new("test", model, 0.01, 32, 10);
        
        // Test train
        let data = vec![1, 2, 3, 4];
        assert!(learner.train(&data).is_ok());
        
        // Test predict
        let result = learner.predict(&data);
        assert!(result.is_ok());
        
        // Test save and load
        assert!(learner.save("/tmp/model").is_ok());
        assert!(learner.load("/tmp/model").is_ok());
        
        // Test reset
        assert!(learner.reset().is_ok());
    }

    #[test]
    fn test_symbolic_learner() {
        let kb = Box::new(SimpleKnowledgeBase::new());
        let mut learner = SymbolicLearner::new("test", kb);
        
        // Test train with simple facts
        let data = b"person(john).\nperson(mary).\nfather(john, bob).";
        assert!(learner.train(data).is_ok());
        
        // Test predict with query
        let query = b"person(?X)";
        let result = learner.predict(query);
        assert!(result.is_ok());
        
        // Test save and load
        assert!(learner.save("/tmp/kb").is_ok());
        assert!(learner.load("/tmp/kb").is_ok());
        
        // Test reset
        assert!(learner.reset().is_ok());
    }

    #[test]
    fn test_learning_error() {
        let neural_error = LearningError::NeuralError(
            crate::neuro_symbolic::neural::NeuralError::ShapeError("Invalid shape".to_string())
        );
        
        let symbolic_error = LearningError::SymbolicError(
            crate::neuro_symbolic::symbolic::SymbolicError::ParseError("Invalid syntax".to_string())
        );
        
        let integration_error = LearningError::IntegrationError("Failed to integrate".to_string());
        let other_error = LearningError::Other("Unknown error".to_string());
        
        assert_eq!(
            format!("{}", neural_error),
            "Neural error: Shape error: Invalid shape"
        );
        
        assert_eq!(
            format!("{}", symbolic_error),
            "Symbolic error: Parse error: Invalid syntax"
        );
        
        assert_eq!(
            format!("{}", integration_error),
            "Integration error: Failed to integrate"
        );
        
        assert_eq!(
            format!("{}", other_error),
            "Other error: Unknown error"
        );
    }

    #[test]
    fn test_error_conversion() {
        let neural_error = crate::neuro_symbolic::neural::NeuralError::ShapeError("Invalid shape".to_string());
        let learning_error: LearningError = neural_error.into();
        
        if let LearningError::NeuralError(_) = learning_error {
            // Conversion successful
        } else {
            panic!("Error conversion failed");
        }
        
        let symbolic_error = crate::neuro_symbolic::symbolic::SymbolicError::ParseError("Invalid syntax".to_string());
        let learning_error: LearningError = symbolic_error.into();
        
        if let LearningError::SymbolicError(_) = learning_error {
            // Conversion successful
        } else {
            panic!("Error conversion failed");
        }
    }

    #[test]
    fn test_module_functions() {
        // Test initialization functions
        assert!(init().is_ok());
        assert!(start().is_ok());
        assert!(stop().is_ok());
    }
}
