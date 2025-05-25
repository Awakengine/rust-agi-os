#!/bin/bash

# AGI操作系统启动脚本
# 此脚本用于在QEMU虚拟机中启动AGI操作系统

# 设置环境变量
export AGI_HOME="/home/ubuntu/agi_vm/rootfs/release"
export LD_LIBRARY_PATH="$AGI_HOME/lib:$LD_LIBRARY_PATH"
export PATH="$AGI_HOME/bin:$PATH"

# 系统配置
CONFIG_FILE="$AGI_HOME/config/system.conf"
LOG_FILE="/var/log/agi_os.log"

# 创建日志目录
mkdir -p /var/log

# 输出启动信息
echo "正在启动AGI操作系统..."
echo "系统版本: 1.0.0"
echo "构建时间: $(date)"
echo "配置文件: $CONFIG_FILE"
echo "日志文件: $LOG_FILE"

# 初始化系统组件
echo "初始化内核组件..."
$AGI_HOME/rust_agi_os --init-kernel

echo "初始化神经符号组件..."
$AGI_HOME/rust_agi_os --init-neuro-symbolic

echo "初始化元推理组件..."
$AGI_HOME/rust_agi_os --init-meta-reasoning

echo "初始化安全组件..."
$AGI_HOME/rust_agi_os --init-security

echo "初始化系统组件..."
$AGI_HOME/rust_agi_os --init-system

echo "初始化交互组件..."
$AGI_HOME/rust_agi_os --init-interaction

echo "初始化反思组件..."
$AGI_HOME/rust_agi_os --init-reflection

# 启动主系统
echo "启动AGI操作系统主程序..."
$AGI_HOME/rust_agi_os --start-system > $LOG_FILE 2>&1 &

# 输出系统状态
echo "AGI操作系统已启动，PID: $!"
echo "系统状态: 运行中"
echo "可通过查看 $LOG_FILE 获取详细日志"
