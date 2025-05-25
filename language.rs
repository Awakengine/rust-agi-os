// AGI操作系统 - 语言处理模块
// 此文件实现自然语言处理功能

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::error::Error;
use std::fmt;

/// 语言处理错误类型
#[derive(Debug)]
pub enum LanguageError {
    InitializationError(String),
    ProcessingError(String),
    ModelLoadError(String),
    InvalidInputError(String),
    ResourceError(String),
}

impl fmt::Display for LanguageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LanguageError::InitializationError(msg) => write!(f, "初始化错误: {}", msg),
            LanguageError::ProcessingError(msg) => write!(f, "处理错误: {}", msg),
            LanguageError::ModelLoadError(msg) => write!(f, "模型加载错误: {}", msg),
            LanguageError::InvalidInputError(msg) => write!(f, "无效输入: {}", msg),
            LanguageError::ResourceError(msg) => write!(f, "资源错误: {}", msg),
        }
    }
}

impl Error for LanguageError {}

/// NLP任务类型
#[derive(Debug, Clone, PartialEq)]
pub enum NLPTask {
    TextClassification,
    NamedEntityRecognition,
    SentimentAnalysis,
    QuestionAnswering,
    TextGeneration,
    Summarization,
    Translation,
    IntentRecognition,
    SyntaxParsing,
    Tokenization,
    Custom(String),
}

/// 语言模型类型
#[derive(Debug, Clone)]
pub struct LanguageModel {
    id: String,
    name: String,
    version: String,
    supported_tasks: Vec<NLPTask>,
    parameters: HashMap<String, String>,
}

impl LanguageModel {
    pub fn new(id: &str, name: &str, version: &str) -> Self {
        LanguageModel {
            id: id.to_string(),
            name: name.to_string(),
            version: version.to_string(),
            supported_tasks: Vec::new(),
            parameters: HashMap::new(),
        }
    }

    pub fn add_supported_task(&mut self, task: NLPTask) -> &mut Self {
        self.supported_tasks.push(task);
        self
    }

    pub fn set_parameter(&mut self, key: &str, value: &str) -> &mut Self {
        self.parameters.insert(key.to_string(), value.to_string());
        self
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_supported_tasks(&self) -> &Vec<NLPTask> {
        &self.supported_tasks
    }

    pub fn supports_task(&self, task: &NLPTask) -> bool {
        self.supported_tasks.contains(task)
    }

    pub fn get_parameter(&self, key: &str) -> Option<&String> {
        self.parameters.get(key)
    }
}

/// 语言处理器配置
#[derive(Debug, Clone)]
pub struct LanguageConfig {
    id: String,
    default_model: Option<String>,
    max_input_length: usize,
    max_output_length: usize,
    default_language: String,
    parameters: HashMap<String, String>,
}

impl LanguageConfig {
    pub fn new(id: &str) -> Self {
        LanguageConfig {
            id: id.to_string(),
            default_model: None,
            max_input_length: 4096,
            max_output_length: 4096,
            default_language: "en".to_string(),
            parameters: HashMap::new(),
        }
    }

    pub fn with_default_model(mut self, model_id: &str) -> Self {
        self.default_model = Some(model_id.to_string());
        self
    }

    pub fn with_max_input_length(mut self, length: usize) -> Self {
        self.max_input_length = length;
        self
    }

    pub fn with_max_output_length(mut self, length: usize) -> Self {
        self.max_output_length = length;
        self
    }

    pub fn with_default_language(mut self, language: &str) -> Self {
        self.default_language = language.to_string();
        self
    }

    pub fn with_parameter(mut self, key: &str, value: &str) -> Self {
        self.parameters.insert(key.to_string(), value.to_string());
        self
    }
}

/// 文本处理器接口
pub trait TextProcessor {
    fn process(&self, text: &str, options: &HashMap<String, String>) -> Result<String, LanguageError>;
    fn get_processor_type(&self) -> &str;
    fn get_supported_languages(&self) -> Vec<String>;
}

/// 意图识别器接口
pub trait IntentRecognizer {
    fn recognize_intent(&self, text: &str) -> Result<(String, f32), LanguageError>;
    fn get_supported_intents(&self) -> Vec<String>;
    fn add_intent_example(&mut self, intent: &str, example: &str) -> Result<(), LanguageError>;
}

/// 实体提取器接口
pub trait EntityExtractor {
    fn extract_entities(&self, text: &str) -> Result<HashMap<String, Vec<String>>, LanguageError>;
    fn get_supported_entity_types(&self) -> Vec<String>;
    fn add_entity_pattern(&mut self, entity_type: &str, pattern: &str) -> Result<(), LanguageError>;
}

/// 语言处理器主类
pub struct LanguageProcessor {
    config: LanguageConfig,
    models: HashMap<String, LanguageModel>,
    text_processors: HashMap<String, Box<dyn TextProcessor + Send + Sync>>,
    intent_recognizers: HashMap<String, Box<dyn IntentRecognizer + Send + Sync>>,
    entity_extractors: HashMap<String, Box<dyn EntityExtractor + Send + Sync>>,
    active_model: Option<String>,
}

impl LanguageProcessor {
    pub fn new(config: LanguageConfig) -> Result<Self, LanguageError> {
        let processor = LanguageProcessor {
            config,
            models: HashMap::new(),
            text_processors: HashMap::new(),
            intent_recognizers: HashMap::new(),
            entity_extractors: HashMap::new(),
            active_model: None,
        };
        
        Ok(processor)
    }

    pub fn add_model(&mut self, model: LanguageModel) -> Result<(), LanguageError> {
        let model_id = model.get_id().to_string();
        self.models.insert(model_id.clone(), model);
        
        // 如果是第一个添加的模型或者匹配默认模型，则设为活动模型
        if self.active_model.is_none() || 
           (self.config.default_model.is_some() && 
            self.config.default_model.as_ref().unwrap() == &model_id) {
            self.active_model = Some(model_id);
        }
        
        Ok(())
    }

    pub fn add_text_processor<T: TextProcessor + Send + Sync + 'static>(
        &mut self, 
        id: &str, 
        processor: T
    ) -> Result<(), LanguageError> {
        self.text_processors.insert(id.to_string(), Box::new(processor));
        Ok(())
    }

    pub fn add_intent_recognizer<T: IntentRecognizer + Send + Sync + 'static>(
        &mut self, 
        id: &str, 
        recognizer: T
    ) -> Result<(), LanguageError> {
        self.intent_recognizers.insert(id.to_string(), Box::new(recognizer));
        Ok(())
    }

    pub fn add_entity_extractor<T: EntityExtractor + Send + Sync + 'static>(
        &mut self, 
        id: &str, 
        extractor: T
    ) -> Result<(), LanguageError> {
        self.entity_extractors.insert(id.to_string(), Box::new(extractor));
        Ok(())
    }

    pub fn set_active_model(&mut self, model_id: &str) -> Result<(), LanguageError> {
        if !self.models.contains_key(model_id) {
            return Err(LanguageError::InvalidInputError(
                format!("模型 '{}' 不存在", model_id)
            ));
        }
        
        self.active_model = Some(model_id.to_string());
        Ok(())
    }

    pub fn get_active_model(&self) -> Option<&LanguageModel> {
        match &self.active_model {
            Some(model_id) => self.models.get(model_id),
            None => None,
        }
    }

    pub fn process_text(
        &self, 
        text: &str, 
        processor_id: &str, 
        options: Option<HashMap<String, String>>
    ) -> Result<String, LanguageError> {
        // 检查输入长度
        if text.len() > self.config.max_input_length {
            return Err(LanguageError::InvalidInputError(
                format!("输入文本超过最大长度限制 {}", self.config.max_input_length)
            ));
        }
        
        // 获取处理器
        let processor = self.text_processors.get(processor_id).ok_or_else(|| {
            LanguageError::InvalidInputError(format!("文本处理器 '{}' 不存在", processor_id))
        })?;
        
        // 处理文本
        let opts = options.unwrap_or_else(HashMap::new);
        processor.process(text, &opts)
    }

    pub fn recognize_intent(
        &self, 
        text: &str, 
        recognizer_id: &str
    ) -> Result<(String, f32), LanguageError> {
        // 检查输入长度
        if text.len() > self.config.max_input_length {
            return Err(LanguageError::InvalidInputError(
                format!("输入文本超过最大长度限制 {}", self.config.max_input_length)
            ));
        }
        
        // 获取意图识别器
        let recognizer = self.intent_recognizers.get(recognizer_id).ok_or_else(|| {
            LanguageError::InvalidInputError(format!("意图识别器 '{}' 不存在", recognizer_id))
        })?;
        
        // 识别意图
        recognizer.recognize_intent(text)
    }

    pub fn extract_entities(
        &self, 
        text: &str, 
        extractor_id: &str
    ) -> Result<HashMap<String, Vec<String>>, LanguageError> {
        // 检查输入长度
        if text.len() > self.config.max_input_length {
            return Err(LanguageError::InvalidInputError(
                format!("输入文本超过最大长度限制 {}", self.config.max_input_length)
            ));
        }
        
        // 获取实体提取器
        let extractor = self.entity_extractors.get(extractor_id).ok_or_else(|| {
            LanguageError::InvalidInputError(format!("实体提取器 '{}' 不存在", extractor_id))
        })?;
        
        // 提取实体
        extractor.extract_entities(text)
    }

    pub fn get_config(&self) -> &LanguageConfig {
        &self.config
    }

    pub fn get_models(&self) -> &HashMap<String, LanguageModel> {
        &self.models
    }

    pub fn get_text_processors(&self) -> Vec<String> {
        self.text_processors.keys().cloned().collect()
    }

    pub fn get_intent_recognizers(&self) -> Vec<String> {
        self.intent_recognizers.keys().cloned().collect()
    }

    pub fn get_entity_extractors(&self) -> Vec<String> {
        self.entity_extractors.keys().cloned().collect()
    }
}

// 实现一个简单的文本处理器示例
pub struct SimpleTextProcessor {
    processor_type: String,
    supported_languages: Vec<String>,
}

impl SimpleTextProcessor {
    pub fn new(processor_type: &str) -> Self {
        SimpleTextProcessor {
            processor_type: processor_type.to_string(),
            supported_languages: vec!["en".to_string(), "zh".to_string()],
        }
    }
}

impl TextProcessor for SimpleTextProcessor {
    fn process(&self, text: &str, options: &HashMap<String, String>) -> Result<String, LanguageError> {
        // 简单示例实现，实际应用中会有更复杂的处理逻辑
        let operation = options.get("operation").unwrap_or(&"none".to_string());
        
        match operation.as_str() {
            "uppercase" => Ok(text.to_uppercase()),
            "lowercase" => Ok(text.to_lowercase()),
            "reverse" => Ok(text.chars().rev().collect()),
            _ => Ok(text.to_string()),
        }
    }

    fn get_processor_type(&self) -> &str {
        &self.processor_type
    }

    fn get_supported_languages(&self) -> Vec<String> {
        self.supported_languages.clone()
    }
}

// 实现一个简单的意图识别器示例
pub struct SimpleIntentRecognizer {
    intent_examples: HashMap<String, Vec<String>>,
}

impl SimpleIntentRecognizer {
    pub fn new() -> Self {
        SimpleIntentRecognizer {
            intent_examples: HashMap::new(),
        }
    }
}

impl IntentRecognizer for SimpleIntentRecognizer {
    fn recognize_intent(&self, text: &str) -> Result<(String, f32), LanguageError> {
        // 简单示例实现，实际应用中会使用更复杂的算法
        let text = text.to_lowercase();
        
        for (intent, examples) in &self.intent_examples {
            for example in examples {
                if text.contains(&example.to_lowercase()) {
                    return Ok((intent.clone(), 0.8));
                }
            }
        }
        
        Ok(("unknown".to_string(), 0.0))
    }

    fn get_supported_intents(&self) -> Vec<String> {
        self.intent_examples.keys().cloned().collect()
    }

    fn add_intent_example(&mut self, intent: &str, example: &str) -> Result<(), LanguageError> {
        let examples = self.intent_examples
            .entry(intent.to_string())
            .or_insert_with(Vec::new);
        
        examples.push(example.to_string());
        Ok(())
    }
}

// 实现一个简单的实体提取器示例
pub struct SimpleEntityExtractor {
    entity_patterns: HashMap<String, Vec<String>>,
}

impl SimpleEntityExtractor {
    pub fn new() -> Self {
        SimpleEntityExtractor {
            entity_patterns: HashMap::new(),
        }
    }
}

impl EntityExtractor for SimpleEntityExtractor {
    fn extract_entities(&self, text: &str) -> Result<HashMap<String, Vec<String>>, LanguageError> {
        let mut result = HashMap::new();
        let text = text.to_lowercase();
        
        for (entity_type, patterns) in &self.entity_patterns {
            let mut entities = Vec::new();
            
            for pattern in patterns {
                if text.contains(&pattern.to_lowercase()) {
                    entities.push(pattern.clone());
                }
            }
            
            if !entities.is_empty() {
                result.insert(entity_type.clone(), entities);
            }
        }
        
        Ok(result)
    }

    fn get_supported_entity_types(&self) -> Vec<String> {
        self.entity_patterns.keys().cloned().collect()
    }

    fn add_entity_pattern(&mut self, entity_type: &str, pattern: &str) -> Result<(), LanguageError> {
        let patterns = self.entity_patterns
            .entry(entity_type.to_string())
            .or_insert_with(Vec::new);
        
        patterns.push(pattern.to_string());
        Ok(())
    }
}
