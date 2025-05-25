// AGI交互模块 - 主程序入口
use agi_interaction::interface::UserInterface;
use agi_interaction::multimodal::MultimodalProcessor;

fn main() {
    let ui = UserInterface::new();
    let processor = MultimodalProcessor::new();
    ui.run(&processor);
}