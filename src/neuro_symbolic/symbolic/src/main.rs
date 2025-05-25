// AGI符号推理模块 - 主程序入口
use agi_symbol::symbolic::SymbolicReasoner;
use agi_symbol::learning::SymbolicLearner;

fn main() {
    let reasoner = SymbolicReasoner::new();
    let learner = SymbolicLearner::new();
    reasoner.run(&learner);
}