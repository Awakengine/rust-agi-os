#!/bin/bash

# QEMU启动脚本 - 用于启动AGI操作系统虚拟机
# 配置为用户指定的高性能硬件：4核8线程CPU、3GB内存、2TB固态硬盘

# 设置虚拟机参数
VM_NAME="AGI_OS"
DISK_IMAGE="/home/ubuntu/agi_vm/agi_os.qcow2"
MEMORY="3G"
CPU_CORES=4
CPU_THREADS=8
CPU_SOCKETS=1

# 网络配置
NET_DEVICE="e1000"
TAP_DEVICE="tap0"

# 启动QEMU虚拟机
qemu-system-x86_64 \
  -name "$VM_NAME" \
  -machine type=q35,accel=kvm \
  -cpu host \
  -smp cores=$CPU_CORES,threads=$((CPU_THREADS/CPU_CORES)),sockets=$CPU_SOCKETS \
  -m $MEMORY \
  -drive file=$DISK_IMAGE,format=qcow2,if=virtio,cache=writeback \
  -netdev user,id=net0 \
  -device $NET_DEVICE,netdev=net0 \
  -display sdl \
  -vga virtio \
  -usb \
  -device usb-tablet \
  -boot order=c \
  -monitor stdio

echo "AGI操作系统虚拟机已关闭"
