//! 系统生命周期管理模块

use anyhow::Result;
use crate::interaction::stop as interaction_stop;
use crate::kernel::stop as kernel_stop;
use crate::meta_reasoning::stop as meta_reasoning_stop;
use crate::neuro_symbolic::stop as neuro_symbolic_stop;
use crate::security::stop as security_stop;

/// 启动整个系统的所有核心模块
pub fn start() -> Result<()> {
    println!("系统所有核心模块启动中...");
    Ok(())
}

/// 停止整个系统的所有核心模块
pub fn stop() -> Result<()> {
    kernel_stop()?;
    neuro_symbolic_stop()?;
    meta_reasoning_stop()?;
    interaction_stop()?;
    security_stop()?;
    println!("系统所有核心模块已成功停止");
    Ok(())
}