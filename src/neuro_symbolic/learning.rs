//! # Learning Module
//! 
//! This module provides advanced learning capabilities for the neural-symbolic system,
//! including incremental learning, meta-learning, and self-improvement mechanisms.

use std::sync::{Arc, Mutex, Once};
use std::collections::HashMap;
use std::fmt;

use crate::neuro_symbolic::{
    neural::{self, NeuralError, NeuralTensor, NeuralModelImpl, NeuralModel},
    symbolic::{self, Fact, Rule, KnowledgeBase, QueryResult, Term, FactSource, SymbolicConfig},
    integration::{self, IntegrationError},
};

// 使用lazy_static替代LazyLock，因为LazyLock在Rust 1.87中可能不可用
use lazy_static::lazy_static;

static INIT: Once = Once::new();

// 全局学习注册表 - 使用lazy_static替代LazyLock
lazy_static! {
    static ref LEARNING_REGISTRY: Mutex<LearningRegistry> = Mutex::new(LearningRegistry::new());
}

/// Initialize the learning subsystem
pub fn init() -> Result<(), LearningError> {
    let mut result = Ok(());
    
    INIT.call_once(|| {
        // Initialize the learning registry
        let _unused = LEARNING_REGISTRY.lock().unwrap();
    });
    
    result
}

/// Error type for learning operations
#[derive(Debug)]
pub enum LearningError {
    /// Neural error
    NeuralError(NeuralError),
    /// Symbolic error
    SymbolicError(symbolic::SymbolicError),
    /// Integration error
    IntegrationError(IntegrationError),
    /// Training error
    TrainingError(String),
    /// Validation error
    ValidationError(String),
    /// General error
    General(&'static str),
}

impl std::error::Error for LearningError {}

impl fmt::Display for LearningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LearningError::NeuralError(err) => write!(f, "Neural error: {}", err),
            LearningError::SymbolicError(err) => write!(f, "Symbolic error: {}", err),
            LearningError::IntegrationError(err) => write!(f, "Integration error: {}", err),
            LearningError::TrainingError(msg) => write!(f, "Training error: {}", msg),
            LearningError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            LearningError::General(msg) => write!(f, "General error: {}", msg),
        }
    }
}

impl From<NeuralError> for LearningError {
    fn from(err: NeuralError) -> Self {
        LearningError::NeuralError(err)
    }
}

impl From<symbolic::SymbolicError> for LearningError {
    fn from(err: symbolic::SymbolicError) -> Self {
        LearningError::SymbolicError(err)
    }
}

impl From<IntegrationError> for LearningError {
    fn from(err: IntegrationError) -> Self {
        LearningError::IntegrationError(err)
    }
}

/// Learning configuration
#[derive(Debug, Clone)]
pub struct LearningConfig {
    /// Enable incremental learning
    pub enable_incremental_learning: bool,
    /// Enable meta-learning
    pub enable_meta_learning: bool,
    /// Enable self-improvement
    pub enable_self_improvement: bool,
    /// Learning rate
    pub learning_rate: f32,
    /// Batch size
    pub batch_size: usize,
    /// Maximum epochs
    pub max_epochs: usize,
    /// Validation split
    pub validation_split: f32,
    /// Early stopping patience
    pub early_stopping_patience: usize,
}

impl Default for LearningConfig {
    fn default() -> Self {
        Self {
            enable_incremental_learning: true,
            enable_meta_learning: true,
            enable_self_improvement: true,
            learning_rate: 0.001,
            batch_size: 32,
            max_epochs: 100,
            validation_split: 0.2,
            early_stopping_patience: 10,
        }
    }
}

/// Learning status
#[derive(Debug, Clone)]
pub struct LearningStatus {
    /// Number of registered learners
    pub learner_count: usize,
    /// Number of training sessions
    pub training_session_count: usize,
    /// Number of successful training sessions
    pub successful_training_count: usize,
    /// Number of failed training sessions
    pub failed_training_count: usize,
    /// Total training time in seconds
    pub total_training_time: f64,
}

/// Training dataset
#[derive(Debug, Clone)]
pub struct Dataset {
    /// Dataset name
    pub name: String,
    /// Dataset description
    pub description: String,
    /// Input tensors
    pub inputs: Vec<NeuralTensor>,
    /// Target tensors (for supervised learning)
    pub targets: Option<Vec<NeuralTensor>>,
    /// Symbolic facts (for neural-symbolic learning)
    pub facts: Option<Vec<Fact>>,
    /// Symbolic rules (for neural-symbolic learning)
    pub rules: Option<Vec<Rule>>,
}

impl Dataset {
    /// Create a new dataset
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            inputs: Vec::new(),
            targets: None,
            facts: None,
            rules: None,
        }
    }
    
    /// Add input tensor
    pub fn add_input(&mut self, input: NeuralTensor) {
        self.inputs.push(input);
    }
    
    /// Set target tensors
    pub fn set_targets(&mut self, targets: Vec<NeuralTensor>) {
        self.targets = Some(targets);
    }
    
    /// Set symbolic facts
    pub fn set_facts(&mut self, facts: Vec<Fact>) {
        self.facts = Some(facts);
    }
    
    /// Set symbolic rules
    pub fn set_rules(&mut self, rules: Vec<Rule>) {
        self.rules = Some(rules);
    }
    
    /// Split dataset into training and validation sets
    pub fn split(&self, validation_ratio: f32) -> (Self, Self) {
        let validation_size = (self.inputs.len() as f32 * validation_ratio) as usize;
        let training_size = self.inputs.len() - validation_size;
        
        let mut training_set = Self::new(&format!("{}_train", self.name), &self.description);
        let mut validation_set = Self::new(&format!("{}_val", self.name), &self.description);
        
        // Split inputs
        training_set.inputs = self.inputs[0..training_size].to_vec();
        validation_set.inputs = self.inputs[training_size..].to_vec();
        
        // Split targets if available
        if let Some(targets) = &self.targets {
            training_set.targets = Some(targets[0..training_size].to_vec());
            validation_set.targets = Some(targets[training_size..].to_vec());
        }
        
        // Split facts if available
        if let Some(facts) = &self.facts {
            training_set.facts = Some(facts[0..training_size.min(facts.len())].to_vec());
            validation_set.facts = Some(facts[training_size.min(facts.len())..].to_vec());
        }
        
        // Copy rules to both sets
        if let Some(rules) = &self.rules {
            training_set.rules = Some(rules.clone());
            validation_set.rules = Some(rules.clone());
        }
        
        (training_set, validation_set)
    }
}

/// Training metrics
#[derive(Debug, Clone)]
pub struct TrainingMetrics {
    /// Loss values per epoch
    pub loss: Vec<f32>,
    /// Accuracy values per epoch (if applicable)
    pub accuracy: Option<Vec<f32>>,
    /// Validation loss values per epoch
    pub val_loss: Vec<f32>,
    /// Validation accuracy values per epoch (if applicable)
    pub val_accuracy: Option<Vec<f32>>,
    /// Training time in seconds
    pub training_time: f64,
    /// Number of epochs
    pub epochs: usize,
}

impl TrainingMetrics {
    /// Create new training metrics
    pub fn new() -> Self {
        Self {
            loss: Vec::new(),
            accuracy: None,
            val_loss: Vec::new(),
            val_accuracy: None,
            training_time: 0.0,
            epochs: 0,
        }
    }
    
    /// Add epoch metrics
    pub fn add_epoch(&mut self, loss: f32, accuracy: Option<f32>, val_loss: f32, val_accuracy: Option<f32>) {
        self.loss.push(loss);
        self.val_loss.push(val_loss);
        
        if let Some(acc) = accuracy {
            if self.accuracy.is_none() {
                self.accuracy = Some(Vec::new());
            }
            self.accuracy.as_mut().unwrap().push(acc);
        }
        
        if let Some(val_acc) = val_accuracy {
            if self.val_accuracy.is_none() {
                self.val_accuracy = Some(Vec::new());
            }
            self.val_accuracy.as_mut().unwrap().push(val_acc);
        }
        
        self.epochs += 1;
    }
    
    /// Get best epoch
    pub fn best_epoch(&self) -> usize {
        // Find the epoch with the lowest validation loss
        self.val_loss.iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(epoch, _)| epoch)
            .unwrap_or(0)
    }
    
    /// Check if training should stop early
    pub fn should_stop_early(&self, patience: usize) -> bool {
        if self.epochs <= patience {
            return false;
        }
        
        let best_epoch = self.best_epoch();
        self.epochs - best_epoch > patience
    }
}

/// Learner trait
pub trait Learner: Send + Sync + fmt::Debug {
    /// Train on a dataset
    fn train(&mut self, dataset: &Dataset, config: &LearningConfig) -> Result<TrainingMetrics, LearningError>;
    
    /// Evaluate on a dataset
    fn evaluate(&self, dataset: &Dataset) -> Result<(f32, Option<f32>), LearningError>;
    
    /// Get learner name
    fn name(&self) -> &str;
    
    /// Get learner description
    fn description(&self) -> &str;
}

/// Neural learner
#[derive(Debug)]
pub struct NeuralLearner {
    /// Learner name
    name: String,
    /// Learner description
    description: String,
    /// Neural model - 使用NeuralModelImpl具体类型而非trait对象
    model: NeuralModelImpl,
    /// Training metrics
    metrics: Option<TrainingMetrics>,
}

impl NeuralLearner {
    /// Create a new neural learner
    pub fn new(name: &str, description: &str, model: NeuralModelImpl) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            model,
            metrics: None,
        }
    }
    
    /// Get training metrics
    pub fn metrics(&self) -> Option<&TrainingMetrics> {
        self.metrics.as_ref()
    }
}

impl Learner for NeuralLearner {
    fn train(&mut self, dataset: &Dataset, config: &LearningConfig) -> Result<TrainingMetrics, LearningError> {
        // Check if dataset has targets
        let targets = dataset.targets.as_ref()
            .ok_or_else(|| LearningError::General("Dataset must have targets for neural learning"))?;
        
        if dataset.inputs.len() != targets.len() {
            return Err(LearningError::General("Number of inputs must match number of targets"));
        }
        
        // Split dataset into training and validation sets
        let (training_set, validation_set) = dataset.split(config.validation_split);
        
        // Initialize metrics
        let mut metrics = TrainingMetrics::new();
        let start_time = std::time::Instant::now();
        
        // Training loop
        for _epoch in 0..config.max_epochs {
            let mut epoch_loss = 0.0;
            let mut epoch_accuracy = 0.0;
            
            // Process mini-batches
            for batch_start in (0..training_set.inputs.len()).step_by(config.batch_size) {
                let batch_end = (batch_start + config.batch_size).min(training_set.inputs.len());
                let batch_size = batch_end - batch_start;
                
                // Prepare batch
                let batch_inputs = &training_set.inputs[batch_start..batch_end];
                let batch_targets = &training_set.targets.as_ref().unwrap()[batch_start..batch_end];
                
                // Forward pass
                let mut batch_outputs = Vec::with_capacity(batch_size);
                for input in batch_inputs {
                    let output = self.model.forward(input)
                        .map_err(|e| LearningError::NeuralError(e))?;
                    batch_outputs.push(output);
                }
                
                // Compute loss
                let mut batch_loss = 0.0;
                for (output, target) in batch_outputs.iter().zip(batch_targets.iter()) {
                    // This is a simplified loss calculation; in a real system, we would use
                    // a proper loss function based on the task
                    let loss = output.data().iter()
                        .zip(target.data().iter())
                        .map(|(o, t)| (o - t).powi(2))
                        .sum::<f32>() / output.data().len() as f32;
                    
                    batch_loss += loss;
                }
                batch_loss /= batch_size as f32;
                
                // Backward pass and update weights
                // In a real system, we would compute gradients and update weights
                // For this prototype, we just simulate the process
                
                epoch_loss += batch_loss;
                
                // Compute accuracy (if applicable)
                // This is a simplified accuracy calculation; in a real system, we would use
                // a proper accuracy metric based on the task
                let batch_accuracy = batch_outputs.iter()
                    .zip(batch_targets.iter())
                    .map(|(output, target)| {
                        let output_argmax = output.data().iter()
                            .enumerate()
                            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                            .map(|(idx, _)| idx)
                            .unwrap_or(0);
                        
                        let target_argmax = target.data().iter()
                            .enumerate()
                            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                            .map(|(idx, _)| idx)
                            .unwrap_or(0);
                        
                        if output_argmax == target_argmax { 1.0 } else { 0.0 }
                    })
                    .sum::<f32>() / batch_size as f32;
                
                epoch_accuracy += batch_accuracy * batch_size as f32;
            }
            
            epoch_loss /= training_set.inputs.len() as f32;
            epoch_accuracy /= training_set.inputs.len() as f32;
            
            // Evaluate on validation set
            let (val_loss, val_accuracy) = self.evaluate(&validation_set)?;
            
            // Update metrics
            metrics.add_epoch(epoch_loss, Some(epoch_accuracy), val_loss, val_accuracy);
            
            // Check for early stopping
            if metrics.should_stop_early(config.early_stopping_patience) {
                break;
            }
        }
        
        // Update training time
        metrics.training_time = start_time.elapsed().as_secs_f64();
        
        // Store metrics
        self.metrics = Some(metrics.clone());
        
        Ok(metrics)
    }
    
    fn evaluate(&self, dataset: &Dataset) -> Result<(f32, Option<f32>), LearningError> {
        // Check if dataset has targets
        let targets = dataset.targets.as_ref()
            .ok_or_else(|| LearningError::General("Dataset must have targets for neural evaluation"))?;
        
        if dataset.inputs.len() != targets.len() {
            return Err(LearningError::General("Number of inputs must match number of targets"));
        }
        
        // Compute loss and accuracy
        let mut total_loss = 0.0;
        let mut total_accuracy = 0.0;
        
        for (input, target) in dataset.inputs.iter().zip(targets.iter()) {
            // Forward pass
            let output = self.model.forward(input)
                .map_err(|e| LearningError::NeuralError(e))?;
            
            // Compute loss
            let loss = output.data().iter()
                .zip(target.data().iter())
                .map(|(o, t)| (o - t).powi(2))
                .sum::<f32>() / output.data(
(Content truncated due to size limit. Use line ranges to read in chunks)