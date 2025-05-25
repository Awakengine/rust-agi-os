mod vision;
mod speech;
mod natural_language;
mod natural_language_processor;
mod language;
mod multimodal;
mod context;
mod interface;

// 使用具体的模块导出，避免glob导出冲突
pub use vision::{VisionSystem, Image, Object, VisionError};
pub use speech::{SpeechSystem, Audio, Recording, SpeechError};
pub use natural_language::{NaturalLanguageSystem, Language, Sentiment, Entity, Intent, NaturalLanguageError};
pub use natural_language_processor::{NaturalLanguageProcessor, NaturalLanguageProcessorError};
pub use language::{LanguageSystem, LanguageError};
pub use multimodal::{MultimodalSystem, MultimodalError};
pub use context::{ContextManager, ContextItem, ContextType, ContextError, ContextWindow, ContextConfig, ContextPriority, MemoryType};
pub use interface::{InterfaceManager, InterfaceConfig, InterfaceType, InterfaceError, UIComponent, EventHandler, InteractionMode, AccessibilityLevel};

// 导出特定函数，避免冲突
pub use vision::init as vision_init;
pub use vision::start as vision_start;
pub use vision::stop as vision_stop;

pub use speech::init as speech_init;
pub use speech::start as speech_start;
pub use speech::stop as speech_stop;

pub use natural_language::init as natural_language_init;
pub use natural_language::start as natural_language_start;
pub use natural_language::stop as natural_language_stop;
