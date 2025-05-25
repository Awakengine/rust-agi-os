use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

/// 用户反馈收集错误类型
#[derive(Debug)]
pub enum FeedbackError {
    /// 初始化错误
    InitializationError(String),
    /// 收集错误
    CollectionError(String),
    /// 分析错误
    AnalysisError(String),
    /// IO错误
    IoError(io::Error),
    /// 其他错误
    Other(String),
}

impl Error for FeedbackError {}

impl fmt::Display for FeedbackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeedbackError::InitializationError(msg) => write!(f, "用户反馈收集初始化错误: {}", msg),
            FeedbackError::CollectionError(msg) => write!(f, "用户反馈收集错误: {}", msg),
            FeedbackError::AnalysisError(msg) => write!(f, "用户反馈分析错误: {}", msg),
            FeedbackError::IoError(err) => write!(f, "IO错误: {}", err),
            FeedbackError::Other(msg) => write!(f, "其他用户反馈错误: {}", msg),
        }
    }
}

impl From<io::Error> for FeedbackError {
    fn from(error: io::Error) -> Self {
        FeedbackError::IoError(error)
    }
}

/// 用户反馈项
#[derive(Debug, Clone)]
pub struct FeedbackItem {
    /// 反馈ID
    pub id: String,
    /// 反馈类型
    pub feedback_type: FeedbackType,
    /// 反馈内容
    pub content: String,
    /// 严重程度
    pub severity: FeedbackSeverity,
    /// 相关模块
    pub module: String,
    /// 时间戳
    pub timestamp: String,
}

/// 反馈类型
#[derive(Debug, Clone, PartialEq)]
pub enum FeedbackType {
    /// 错误报告
    BugReport,
    /// 功能建议
    FeatureRequest,
    /// 性能问题
    PerformanceIssue,
    /// 用户体验
    UserExperience,
    /// 其他
    Other,
}

impl FeedbackType {
    pub fn as_str(&self) -> &str {
        match self {
            FeedbackType::BugReport => "错误报告",
            FeedbackType::FeatureRequest => "功能建议",
            FeedbackType::PerformanceIssue => "性能问题",
            FeedbackType::UserExperience => "用户体验",
            FeedbackType::Other => "其他",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "错误报告" | "bug" | "bug report" => FeedbackType::BugReport,
            "功能建议" | "feature" | "feature request" => FeedbackType::FeatureRequest,
            "性能问题" | "performance" | "performance issue" => FeedbackType::PerformanceIssue,
            "用户体验" | "ux" | "user experience" => FeedbackType::UserExperience,
            _ => FeedbackType::Other,
        }
    }
}

/// 反馈严重程度
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum FeedbackSeverity {
    /// 关键
    Critical,
    /// 高
    High,
    /// 中
    Medium,
    /// 低
    Low,
    /// 建议
    Suggestion,
}

impl FeedbackSeverity {
    pub fn as_str(&self) -> &str {
        match self {
            FeedbackSeverity::Critical => "关键",
            FeedbackSeverity::High => "高",
            FeedbackSeverity::Medium => "中",
            FeedbackSeverity::Low => "低",
            FeedbackSeverity::Suggestion => "建议",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "关键" | "critical" => FeedbackSeverity::Critical,
            "高" | "high" => FeedbackSeverity::High,
            "中" | "medium" => FeedbackSeverity::Medium,
            "低" | "low" => FeedbackSeverity::Low,
            _ => FeedbackSeverity::Suggestion,
        }
    }
}

/// 用户反馈收集器
pub struct FeedbackCollector {
    /// 反馈项
    feedback_items: Vec<FeedbackItem>,
    /// 收集路径
    collection_path: String,
    /// 日志
    logs: Vec<String>,
}

impl FeedbackCollector {
    pub fn new(collection_path: &str) -> Self {
        Self {
            feedback_items: Vec::new(),
            collection_path: collection_path.to_string(),
            logs: Vec::new(),
        }
    }
    
    /// 记录日志
    fn log(&mut self, message: &str) {
        println!("[FEEDBACK] {}", message);
        self.logs.push(format!("[{}] {}", chrono::Local::now().format("%H:%M:%S"), message));
    }
    
    /// 获取所有日志
    pub fn get_logs(&self) -> &[String] {
        &self.logs
    }
    
    /// 清空日志
    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }
    
    /// 添加反馈项
    pub fn add_feedback(&mut self, item: FeedbackItem) {
        self.log(&format!(
            "添加反馈: [{}] {} ({})",
            item.feedback_type.as_str(),
            item.content,
            item.severity.as_str()
        ));
        
        self.feedback_items.push(item);
    }
    
    /// 获取所有反馈项
    pub fn get_feedback_items(&self) -> &[FeedbackItem] {
        &self.feedback_items
    }
    
    /// 获取特定类型的反馈项
    pub fn get_feedback_by_type(&self, feedback_type: FeedbackType) -> Vec<&FeedbackItem> {
        self.feedback_items
            .iter()
            .filter(|item| item.feedback_type == feedback_type)
            .collect()
    }
    
    /// 获取特定严重程度的反馈项
    pub fn get_feedback_by_severity(&self, severity: FeedbackSeverity) -> Vec<&FeedbackItem> {
        self.feedback_items
            .iter()
            .filter(|item| item.severity == severity)
            .collect()
    }
    
    /// 获取特定模块的反馈项
    pub fn get_feedback_by_module(&self, module: &str) -> Vec<&FeedbackItem> {
        self.feedback_items
            .iter()
            .filter(|item| item.module == module)
            .collect()
    }
    
    /// 收集用户反馈
    pub fn collect_feedback(&mut self) -> Result<(), FeedbackError> {
        self.log("开始收集用户反馈...");
        
        // 确保收集路径存在
        if !Path::new(&self.collection_path).exists() {
            fs::create_dir_all(&self.collection_path)?;
        }
        
        // 收集反馈文件
        let feedback_files = fs::read_dir(&self.collection_path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "feedback") {
                    Some(path)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
            
        self.log(&format!("找到 {} 个反馈文件", feedback_files.len()));
        
        // 解析反馈文件
        for path in feedback_files {
            self.log(&format!("解析反馈文件: {:?}", path));
            
            let content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(err) => {
                    self.log(&format!("无法读取反馈文件 {:?}: {}", path, err));
                    continue;
                }
            };
            
            let lines = content.lines().collect::<Vec<_>>();
            if lines.len() < 4 {
                self.log(&format!("反馈文件 {:?} 格式不正确", path));
                continue;
            }
            
            let id = path.file_stem().unwrap().to_string_lossy().to_string();
            let feedback_type = FeedbackType::from_str(lines[0]);
            let severity = FeedbackSeverity::from_str(lines[1]);
            let module = lines[2].to_string();
            let content = lines[3..].join("\n");
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            
            let item = FeedbackItem {
                id,
                feedback_type,
                content,
                severity,
                module,
                timestamp,
            };
            
            self.add_feedback(item);
        }
        
        self.log(&format!("成功收集 {} 个反馈项", self.feedback_items.len()));
        
        Ok(())
    }
    
    /// 分析反馈
    pub fn analyze_feedback(&self) -> Result<FeedbackAnalysis, FeedbackError> {
        if self.feedback_items.is_empty() {
            return Err(FeedbackError::AnalysisError("没有反馈项可供分析".to_string()));
        }
        
        let mut analysis = FeedbackAnalysis::new();
        
        // 按类型统计
        for item in &self.feedback_items {
            match item.feedback_type {
                FeedbackType::BugReport => analysis.bug_reports += 1,
                FeedbackType::FeatureRequest => analysis.feature_requests += 1,
                FeedbackType::PerformanceIssue => analysis.performance_issues += 1,
                FeedbackType::UserExperience => analysis.user_experience_issues += 1,
                FeedbackType::Other => analysis.other_issues += 1,
            }
            
            match item.severity {
                FeedbackSeverity::Critical => analysis.critical_issues += 1,
                FeedbackSeverity::High => analysis.high_severity_issues += 1,
                FeedbackSeverity::Medium => analysis.medium_severity_issues += 1,
                FeedbackSeverity::Low => analysis.low_severity_issues += 1,
                FeedbackSeverity::Suggestion => analysis.suggestions += 1,
            }
            
            // 统计模块
            let entry = analysis.module_issues.entry(item.module.clone()).or_insert(0);
            *entry += 1;
        }
        
        // 找出最常见的问题
        if !self.feedback_items.is_empty() {
            let mut content_count = std::collections::HashMap::new();
            for item in &self.feedback_items {
                let entry = content_count.entry(item.content.clone()).or_insert(0);
                *entry += 1;
            }
            
            let mut content_vec = content_count.into_iter().collect::<Vec<_>>();
            content_vec.sort_by(|a, b| b.1.cmp(&a.1));
            
            if !content_vec.is_empty() {
                analysis.most_common_issue = Some(content_vec[0].0.clone());
                analysis.most_common_issue_count = content_vec[0].1;
            }
        }
        
        // 找出最需要关注的模块
        if !analysis.module_issues.is_empty() {
            let mut module_vec = analysis.module_issues.clone().into_iter().collect::<Vec<_>>();
            module_vec.sort_by(|a, b| b.1.cmp(&a.1));
            
            if !module_vec.is_empty() {
                analysis.most_problematic_module = Some(module_vec[0].0.clone());
                analysis.most_problematic_module_issues = module_vec[0].1;
            }
        }
        
        Ok(analysis)
    }
    
    /// 生成反馈报告
    pub fn generate_feedback_report(&self) -> Result<String, FeedbackError> {
        let report_path = format!("{}/feedback_report.md", self.collection_path);
        let mut file = File::create(&report_path)?;
        
        writeln!(file, "# 用户反馈报告")?;
        writeln!(file)?;
        writeln!(file, "## 反馈摘要")?;
        writeln!(file)?;
        
        if self.feedback_items.is_empty() {
            writeln!(file, "没有收集到用户反馈。")?;
        } else {
            let analysis = self.analyze_feedback()?;
            
            writeln!(file, "- 总反馈数: {}", self.feedback_items.len())?;
            writeln!(file, "- 错误报告: {}", analysis.bug_reports)?;
            writeln!(file, "- 功能建议: {}", analysis.feature_requests)?;
            writeln!(file, "- 性能问题: {}", analysis.performance_issues)?;
            writeln!(file, "- 用户体验问题: {}", analysis.user_experience_issues)?;
            writeln!(file, "- 其他问题: {}", analysis.other_issues)?;
            writeln!(file)?;
            
            writeln!(file, "### 严重程度分布")?;
            writeln!(file)?;
            writeln!(file, "- 关键问题: {}", analysis.critical_issues)?;
            writeln!(file, "- 高严重度问题: {}", analysis.high_severity_issues)?;
            writeln!(file, "- 中严重度问题: {}", analysis.medium_severity_issues)?;
            writeln!(file, "- 低严重度问题: {}", analysis.low_severity_issues)?;
            writeln!(file, "- 建议: {}", analysis.suggestions)?;
            writeln!(file)?;
            
            if let Some(issue) = &analysis.most_common_issue {
                writeln!(file, "### 最常见问题")?;
                writeln!(file)?;
                writeln!(file, "- 问题: {}", issue)?;
                writeln!(file, "- 出现次数: {}", analysis.most_common_issue_count)?;
                writeln!(file)?;
            }
            
            if let Some(module) = &analysis.most_problematic_module {
                writeln!(file, "### 最需要关注的模块")?;
                writeln!(file)?;
                writeln!(file, "- 模块: {}", module)?;
                writeln!(file, "- 问题数: {}", analysis.most_problematic_module_issues)?;
                writeln!(file)?;
            }
            
            writeln!(file, "### 模块问题分布")?;
            writeln!(file)?;
            
            let mut module_vec = analysis.module_issues.into_iter().collect::<Vec<_>>();
            module_vec.sort_by(|a, b| b.1.cmp(&a.1));
            
            for (module, count) in module_vec {
                writeln!(file, "- {}: {} 个问题", module, count)?;
            }
            
            writeln!(file)?;
            writeln!(file, "## 详细反馈列表")?;
            writeln!(file)?;
            
            // 按严重程度排序
            let mut sorted_items = self.feedback_items.clone();
            sorted_items.sort_by(|a, b| {
                let a_value = match a.severity {
                    FeedbackSeverity::Critical => 0,
                    FeedbackSeverity::High => 1,
                    FeedbackSeverity::Medium => 2,
                    FeedbackSeverity::Low => 3,
                    FeedbackSeverity::Suggestion => 4,
                };
                
                let b_value = match b.severity {
                    FeedbackSeverity::Critical => 0,
                    FeedbackSeverity::High => 1,
                    FeedbackSeverity::Medium => 2,
                    FeedbackSeverity::Low => 3,
                    FeedbackSeverity::Suggestion => 4,
                };
                
                a_value.cmp(&b_value)
            });
            
            for (i, item) in sorted_items.iter().enumerate() {
                writeln!(file, "### {}. {} ({})", i + 1, item.feedback_type.as_str(), item.severity.as_str())?;
                writeln!(file, "- ID: {}", item.id)?;
                writeln!(file, "- 模块: {}", item.module)?;
                writeln!(file, "- 时间: {}", item.timestamp)?;
                writeln!(file, "- 内容:")?;
                writeln!(file, "```")?;
                writeln!(file, "{}", item.content)?;
                writeln!(file, "```")?;
                writeln!(file)?;
            }
        }
        
        Ok(report_path)
    }
    
    /// 应用反馈优化
    pub fn apply_feedback_optimizations(&self) -> Result<Vec<String>, FeedbackError> {
        if self.feedback_items.is_empty() {
            return Ok(Vec::new());
        }
        
        let mut optimizations = Vec::new();
        
        // 处理关键和高严重度问题
        let critical_issues = self.get_feedback_by_severity(FeedbackSeverity::Critical);
        let high_issues = self.get_feedback_by_severity(FeedbackSeverity::High);
        
        for issue in critical_issues.iter().chain(high_issues.iter()) {
            let optimization = format!(
                "针对 [{}] {} 的优化: 修复 {} 模块中的问题",
                issue.severity.as_str(),
                issue.feedback_type.as_str(),
                issue.module
            );
            
            optimizations.push(optimization);
        }
        
        // 处理性能问题
        let performance_issues = self.get_feedback_by_type(FeedbackType::PerformanceIssue);
        for issue in &performance_issues {
            let optimization = format!(
                "性能优化: 改进 {} 模块的性能",
                issue.module
            );
            
            optimizations.push(optimization);
        }
        
        // 处理用户体验问题
        let ux_issues = self.get_feedback_by_type(FeedbackType::UserExperience);
        for issue in &ux_issues {
            let optimization = format!(
                "用户体验优化: 改进 {} 模块的用户体验",
                issue.module
            );
            
            optimizations.push(optimization);
        }
        
        Ok(optimizations)
    }
}

/// 反馈分析结果
#[derive(Debug)]
pub struct FeedbackAnalysis {
    /// 错误报告数量
    pub bug_reports: usize,
    /// 功能建议数量
    pub feature_requests: usize,
    /// 性能问题数量
    pub performance_issues: usize,
    /// 用户体验问题数量
    pub user_experience_issues: usize,
    /// 其他问题数量
    pub other_issues: usize,
    /// 关键问题数量
    pub critical_issues: usize,
    /// 高严重度问题数量
    pub high_severity_issues: usize,
    /// 中严重度问题数量
    pub medium_severity_issues: usize,
    /// 低严重度问题数量
    pub low_severity_issues: usize,
    /// 建议数量
    pub suggestions: usize,
    /// 模块问题数量
    pub module_issues: std::collections::HashMap<String, usize>,
    /// 最常见的问题
    pub most_common_issue: Option<String>,
    /// 最常见问题出现次数
    pub most_common_issue_count: usize,
    /// 最需要关注的模块
    pub most_problematic_module: Option<String>,
    /// 最需要关注模块的问题数
    pub most_problematic_module_issues: usize,
}

impl FeedbackAnalysis {
    pub fn new() -> Self {
        Self {
            bug_reports: 0,
            feature_requests: 0,
            performance_issues: 0,
            user_experience_issues: 0,
            other_issues: 0,
            critical_issues: 0,
            high_severity_issues: 0,
            medium_severity_issues: 0,
            low_severity_issues: 0,
            suggestions: 0,
            module_issues: std::collections::HashMap::new(),
            most_common_issue: None,
            most_common_issue_count: 0,
            most_problematic_module: None,
            most_problematic_module_issues: 0,
        }
    }
}

/// 创建反馈收集器
pub fn create_feedback_collector(collection_path: &str) -> FeedbackCollector {
    FeedbackCollector::new(collection_path)
}

/// 收集并分析用户反馈
pub fn collect_and_analyze_feedback(collection_path: &str) -> Result<String, Box<dyn Error>> {
    let mut collector = create_feedback_collector(collection_path);
    
    // 收集反馈
    collector.collect_feedback()?;
    
    // 生成报告
    let report_path = collector.generate_feedback_report()?;
    
    // 应用优化
    let optimizations = collector.apply_feedback_optimizations()?;
    
    // 将优化写入文件
    if !optimizations.is_empty() {
        let optimizations_path = format!("{}/optimizations.md", collection_path);
        let mut file = File::create(&optimizations_path)?;
        
        writeln!(file, "# 基于用户反馈的优化建议")?;
        writeln!(file)?;
        
        for (i, optimization) in optimizations.iter().enumerate() {
            writeln!(file, "{}. {}", i + 1, optimization)?;
        }
    }
    
    Ok(report_path)
}
