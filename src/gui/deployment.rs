use std::error::Error;
use std::fmt;
use std::path::Path;
use std::process::Command;
use std::fs::{self, File};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use crate::gui::window::WindowManager;
use crate::gui::high_dpi::HighDpiScalingManager;
use crate::gui::input_method::InputMethodManager;
use crate::gui::theme::ThemeManager;
use crate::gui::keyboard_input::KeyboardInputManager;
use crate::gui::mouse_input::MouseInputManager;

/// 部署错误类型
#[derive(Debug)]
pub enum DeploymentError {
    /// 初始化错误
    InitializationError(String),
    /// 打包错误
    PackagingError(String),
    /// 部署错误
    DeploymentError(String),
    /// 虚拟机错误
    VirtualMachineError(String),
    /// IO错误
    IoError(io::Error),
    /// 其他错误
    Other(String),
}

impl Error for DeploymentError {}

impl fmt::Display for DeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeploymentError::InitializationError(msg) => write!(f, "部署初始化错误: {}", msg),
            DeploymentError::PackagingError(msg) => write!(f, "打包错误: {}", msg),
            DeploymentError::DeploymentError(msg) => write!(f, "部署错误: {}", msg),
            DeploymentError::VirtualMachineError(msg) => write!(f, "虚拟机错误: {}", msg),
            DeploymentError::IoError(err) => write!(f, "IO错误: {}", err),
            DeploymentError::Other(msg) => write!(f, "其他部署错误: {}", msg),
        }
    }
}

impl From<io::Error> for DeploymentError {
    fn from(error: io::Error) -> Self {
        DeploymentError::IoError(error)
    }
}

/// 虚拟机类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtualMachineType {
    /// VMware
    VMware,
    /// VirtualBox
    VirtualBox,
    /// QEMU/KVM
    QEMU,
    /// Hyper-V
    HyperV,
}

impl VirtualMachineType {
    pub fn as_str(&self) -> &'static str {
        match self {
            VirtualMachineType::VMware => "VMware",
            VirtualMachineType::VirtualBox => "VirtualBox",
            VirtualMachineType::QEMU => "QEMU/KVM",
            VirtualMachineType::HyperV => "Hyper-V",
        }
    }
}

/// 部署配置
pub struct DeploymentConfig {
    /// 输出目录
    output_dir: String,
    /// 虚拟机类型
    vm_type: VirtualMachineType,
    /// 虚拟机磁盘路径
    vm_disk_path: Option<String>,
    /// 虚拟机配置文件路径
    vm_config_path: Option<String>,
    /// 是否启用硬件加速
    enable_hardware_acceleration: bool,
    /// 是否启用4K分辨率
    enable_4k_resolution: bool,
    /// 是否启用输入设备直通
    enable_input_passthrough: bool,
}

impl DeploymentConfig {
    pub fn new(output_dir: &str, vm_type: VirtualMachineType) -> Self {
        Self {
            output_dir: output_dir.to_string(),
            vm_type,
            vm_disk_path: None,
            vm_config_path: None,
            enable_hardware_acceleration: true,
            enable_4k_resolution: true,
            enable_input_passthrough: true,
        }
    }
    
    /// 设置虚拟机磁盘路径
    pub fn set_vm_disk_path(&mut self, path: &str) {
        self.vm_disk_path = Some(path.to_string());
    }
    
    /// 设置虚拟机配置文件路径
    pub fn set_vm_config_path(&mut self, path: &str) {
        self.vm_config_path = Some(path.to_string());
    }
    
    /// 设置是否启用硬件加速
    pub fn set_hardware_acceleration(&mut self, enable: bool) {
        self.enable_hardware_acceleration = enable;
    }
    
    /// 设置是否启用4K分辨率
    pub fn set_4k_resolution(&mut self, enable: bool) {
        self.enable_4k_resolution = enable;
    }
    
    /// 设置是否启用输入设备直通
    pub fn set_input_passthrough(&mut self, enable: bool) {
        self.enable_input_passthrough = enable;
    }
}

/// 部署管理器
pub struct DeploymentManager {
    /// 部署配置
    config: DeploymentConfig,
    /// 日志
    logs: Vec<String>,
}

impl DeploymentManager {
    pub fn new(config: DeploymentConfig) -> Self {
        Self {
            config,
            logs: Vec::new(),
        }
    }
    
    /// 记录日志
    fn log(&mut self, message: &str) {
        println!("[DEPLOY] {}", message);
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
    
    /// 创建输出目录
    fn create_output_directory(&mut self) -> Result<(), DeploymentError> {
        self.log(&format!("创建输出目录: {}", self.config.output_dir));
        
        fs::create_dir_all(&self.config.output_dir)?;
        
        self.log("输出目录创建成功");
        
        Ok(())
    }
    
    /// 打包应用
    pub fn package_application(&mut self, source_dir: &str) -> Result<String, DeploymentError> {
        self.log(&format!("开始打包应用: {}", source_dir));
        
        // 创建输出目录
        self.create_output_directory()?;
        
        // 构建应用
        self.log("构建应用...");
        let status = Command::new("cargo")
            .args(&["build", "--release"])
            .current_dir(source_dir)
            .status()?;
            
        if !status.success() {
            return Err(DeploymentError::PackagingError("构建应用失败".to_string()));
        }
        
        self.log("应用构建成功");
        
        // 复制构建产物到输出目录
        let target_dir = format!("{}/target/release", source_dir);
        let output_bin_dir = format!("{}/bin", self.config.output_dir);
        
        fs::create_dir_all(&output_bin_dir)?;
        
        self.log(&format!("复制构建产物到: {}", output_bin_dir));
        
        // 复制可执行文件
        let executable_name = Path::new(source_dir)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("rust_agi_os");
            
        let source_executable = format!("{}/{}", target_dir, executable_name);
        let target_executable = format!("{}/{}", output_bin_dir, executable_name);
        
        fs::copy(&source_executable, &target_executable)?;
        
        // 复制配置文件和资源
        let output_config_dir = format!("{}/config", self.config.output_dir);
        let output_resources_dir = format!("{}/resources", self.config.output_dir);
        
        fs::create_dir_all(&output_config_dir)?;
        fs::create_dir_all(&output_resources_dir)?;
        
        // 创建启动脚本
        let startup_script = format!("{}/start.sh", self.config.output_dir);
        let mut file = File::create(&startup_script)?;
        
        writeln!(file, "#!/bin/bash")?;
        writeln!(file, "# 启动 Rust AGI OS GUI 桌面环境")?;
        writeln!(file, "cd \"$(dirname \"$0\")\"")?;
        writeln!(file, "export DISPLAY=:0")?;
        writeln!(file, "export RUST_AGI_OS_CONFIG=\"$(pwd)/config\"")?;
        writeln!(file, "export RUST_AGI_OS_RESOURCES=\"$(pwd)/resources\"")?;
        
        if self.config.enable_hardware_acceleration {
            writeln!(file, "export RUST_AGI_OS_HARDWARE_ACCELERATION=1")?;
        }
        
        if self.config.enable_4k_resolution {
            writeln!(file, "export RUST_AGI_OS_RESOLUTION=3840x2160")?;
        } else {
            writeln!(file, "export RUST_AGI_OS_RESOLUTION=1920x1080")?;
        }
        
        writeln!(file, "bin/{} \"$@\"", executable_name)?;
        
        // 设置脚本为可执行
        let status = Command::new("chmod")
            .args(&["+x", &startup_script])
            .status()?;
            
        if !status.success() {
            return Err(DeploymentError::PackagingError("设置启动脚本为可执行失败".to_string()));
        }
        
        self.log("创建启动脚本成功");
        
        // 创建虚拟机配置文件
        self.create_vm_config()?;
        
        self.log("应用打包完成");
        
        Ok(self.config.output_dir.clone())
    }
    
    /// 创建虚拟机配置文件
    fn create_vm_config(&mut self) -> Result<(), DeploymentError> {
        self.log("创建虚拟机配置文件...");
        
        let vm_config_dir = format!("{}/vm", self.config.output_dir);
        fs::create_dir_all(&vm_config_dir)?;
        
        match self.config.vm_type {
            VirtualMachineType::VMware => {
                let vmx_path = format!("{}/rust_agi_os.vmx", vm_config_dir);
                let mut file = File::create(&vmx_path)?;
                
                writeln!(file, ".encoding = \"UTF-8\"")?;
                writeln!(file, "config.version = \"8\"")?;
                writeln!(file, "virtualHW.version = \"18\"")?;
                writeln!(file, "displayName = \"Rust AGI OS GUI\"")?;
                writeln!(file, "guestOS = \"ubuntu-64\"")?;
                writeln!(file, "memsize = \"4096\"")?;
                writeln!(file, "numvcpus = \"4\"")?;
                
                if self.config.enable_hardware_acceleration {
                    writeln!(file, "mks.enable3d = \"TRUE\"")?;
                    writeln!(file, "svga.vramSize = \"268435456\"")?;
                    writeln!(file, "vhv.enable = \"TRUE\"")?;
                }
                
                if self.config.enable_4k_resolution {
                    writeln!(file, "svga.maxWidth = \"3840\"")?;
                    writeln!(file, "svga.maxHeight = \"2160\"")?;
                }
                
                if self.config.enable_input_passthrough {
                    writeln!(file, "usb.present = \"TRUE\"")?;
                    writeln!(file, "usb.generic.autoconnect = \"TRUE\"")?;
                }
                
                if let Some(disk_path) = &self.config.vm_disk_path {
                    writeln!(file, "scsi0.present = \"TRUE\"")?;
                    writeln!(file, "scsi0.virtualDev = \"lsilogic\"")?;
                    writeln!(file, "scsi0:0.present = \"TRUE\"")?;
                    writeln!(file, "scsi0:0.fileName = \"{}\"", disk_path)?;
                }
                
                self.log(&format!("VMware配置文件创建成功: {}", vmx_path));
            },
            VirtualMachineType::VirtualBox => {
                let vbox_path = format!("{}/rust_agi_os.vbox", vm_config_dir);
                let mut file = File::create(&vbox_path)?;
                
                // 简化的VirtualBox配置文件
                writeln!(file, "<?xml version=\"1.0\"?>")?;
                writeln!(file, "<VirtualBox xmlns=\"http://www.virtualbox.org/\" version=\"1.16-linux\">")?;
                writeln!(file, "  <Machine name=\"Rust AGI OS GUI\" OSType=\"Ubuntu_64\" snapshotFolder=\"Snapshots\">")?;
                writeln!(file, "    <Hardware>")?;
                writeln!(file, "      <CPU count=\"4\"/>")?;
                writeln!(file, "      <Memory RAMSize=\"4096\"/>")?;
                writeln!(file, "      <Display VRAMSize=\"128\" accelerate3D=\"{}\" accelerate2DVideo=\"{}\"/>",
                    self.config.enable_hardware_acceleration, self.config.enable_hardware_acceleration)?;
                
                if self.config.enable_4k_resolution {
                    writeln!(file, "      <VideoCapture screens=\"1\" file=\".\" fps=\"25\"/>")?;
                    writeln!(file, "      <RemoteDisplay enabled=\"false\"/>")?;
                }
                
                writeln!(file, "    </Hardware>")?;
                writeln!(file, "  </Machine>")?;
                writeln!(file, "</VirtualBox>")?;
                
                self.log(&format!("VirtualBox配置文件创建成功: {}", vbox_path));
            },
            VirtualMachineType::QEMU => {
                let qemu_script = format!("{}/start_qemu.sh", vm_config_dir);
                let mut file = File::create(&qemu_script)?;
                
                writeln!(file, "#!/bin/bash")?;
                writeln!(file, "# QEMU启动脚本")?;
                
                let mut qemu_args = vec![
                    "-name \"Rust AGI OS GUI\"".to_string(),
                    "-m 4G".to_string(),
                    "-smp 4".to_string(),
                ];
                
                if self.config.enable_hardware_acceleration {
                    qemu_args.push("-device virtio-vga-gl".to_string());
                    qemu_args.push("-display gtk,gl=on".to_string());
                } else {
                    qemu_args.push("-device virtio-vga".to_string());
                    qemu_args.push("-display gtk".to_string());
                }
                
                if self.config.enable_4k_resolution {
                    qemu_args.push("-device virtio-vga,xres=3840,yres=2160".to_string());
                }
                
                if let Some(disk_path) = &self.config.vm_disk_path {
                    qemu_args.push(format!("-drive file={},format=qcow2", disk_path));
                }
                
                writeln!(file, "qemu-system-x86_64 \\")?;
                for arg in &qemu_args[..qemu_args.len() - 1] {
                    writeln!(file, "  {} \\", arg)?;
                }
                writeln!(file, "  {}", qemu_args.last().unwrap())?;
                
                // 设置脚本为可执行
                let status = Command::new("chmod")
                    .args(&["+x", &qemu_script])
                    .status()?;
                    
                if !status.success() {
                    return Err(DeploymentError::PackagingError("设置QEMU启动脚本为可执行失败".to_string()));
                }
                
                self.log(&format!("QEMU启动脚本创建成功: {}", qemu_script));
            },
            VirtualMachineType::HyperV => {
                let hyperv_script = format!("{}/create_hyperv.ps1", vm_config_dir);
                let mut file = File::create(&hyperv_script)?;
                
                writeln!(file, "# Hyper-V创建脚本")?;
                writeln!(file, "New-VM -Name \"Rust AGI OS GUI\" -MemoryStartupBytes 4GB -Generation 2")?;
                writeln!(file, "Set-VMProcessor -VMName \"Rust AGI OS GUI\" -Count 4")?;
                
                if let Some(disk_path) = &self.config.vm_disk_path {
                    writeln!(file, "Add-VMHardDiskDrive -VMName \"Rust AGI OS GUI\" -Path \"{}\"", disk_path)?;
                }
                
                if self.config.enable_4k_resolution {
                    writeln!(file, "Set-VMVideo -VMName \"Rust AGI OS GUI\" -ResolutionType Single -HorizontalResolution 3840 -VerticalResolution 2160")?;
                }
                
                self.log(&format!("Hyper-V创建脚本创建成功: {}", hyperv_script));
            },
        }
        
        self.log("虚拟机配置文件创建完成");
        
        Ok(())
    }
    
    /// 部署到虚拟机
    pub fn deploy_to_vm(&mut self, package_dir: &str) -> Result<(), DeploymentError> {
        self.log(&format!("开始部署到{}虚拟机...", self.config.vm_type.as_str()));
        
        // 检查虚拟机磁盘路径
        if self.config.vm_disk_path.is_none() {
            self.log("警告: 未指定虚拟机磁盘路径，跳过部署到虚拟机");
            return Ok(());
        }
        
        match self.config.vm_type {
            VirtualMachineType::VMware => {
                self.deploy_to_vmware(package_dir)?;
            },
            VirtualMachineType::VirtualBox => {
                self.deploy_to_virtualbox(package_dir)?;
            },
            VirtualMachineType::QEMU => {
                self.deploy_to_qemu(package_dir)?;
            },
            VirtualMachineType::HyperV => {
                self.deploy_to_hyperv(package_dir)?;
            },
        }
        
        self.log("部署到虚拟机完成");
        
        Ok(())
    }
    
    /// 部署到VMware
    fn deploy_to_vmware(&mut self, package_dir: &str) -> Result<(), DeploymentError> {
        self.log("部署到VMware虚拟机...");
        
        // 检查VMware工具是否可用
        let vmrun_status = Command::new("which")
            .arg("vmrun")
            .status();
            
        if vmrun_status.is_err() || !vmrun_status.unwrap().success() {
            self.log("警告: VMware工具(vmrun)不可用，无法自动部署");
            return Ok(());
        }
        
        let vm_config_path = format!("{}/vm/rust_agi_os.vmx", self.config.output_dir);
        
        // 启动虚拟机
        self.log("启动虚拟机...");
        let status = Command::new("vmrun")
            .args(&["start", &vm_config_path])
            .status()?;
            
        if !status.success() {
            return Err(DeploymentError::VirtualMachineError("启动VMware虚拟机失败".to_string()));
        }
        
        // 等待虚拟机启动
        std::thread::sleep(std::time::Duration::from_secs(30));
        
        // 复制文件到虚拟机
        self.log("复制文件到虚拟机...");
        let status = Command::new("vmrun")
            .args(&[
                "copyFileFromHostToGuest",
                &vm_config_path,
                package_dir,
                "/opt/rust_agi_os",
            ])
            .status()?;
            
        if !status.success() {
            return Err(DeploymentError::VirtualMachineError("复制文件到VMware虚拟机失败".to_string()));
        }
        
        // 在虚拟机中运行启动脚本
        self.log("在虚拟机中运行启动脚本...");
        let status = Command::new("vmrun")
            .args(&[
                "runProgramInGuest",
                &vm_config_path,
                "/bin/bash",
                "/opt/rust_agi_os/start.sh",
            ])
            .status()?;
            
        if !status.success() {
            return Err(DeploymentError::VirtualMachineError("在VMware虚拟机中运行启动脚本失败".to_string()));
        }
        
        self.log("VMware虚拟机部署完成");
        
        Ok(())
    }
    
    /// 部署到VirtualBox
    fn deploy_to_virtualbox(&mut self, package_dir: &str) -> Result<(), DeploymentError> {
        self.log("部署到VirtualBox虚拟机...");
        
        // 检查VirtualBox工具是否可用
        let vboxmanage_status = Command::new("which")
            .arg("VBoxManage")
            .status();
            
        if vboxmanage_status.is_err() || !vboxmanage_status.unwrap().success() {
            self.log("警告: VirtualBox工具(VBoxManage)不可用，无法自动部署");
            return Ok(());
        }
        
        // 导入虚拟机
        self.log("导入虚拟机...");
        let vbox_path = format!("{}/vm/rust_agi_os.vbox", self.config.output_dir);
        let status = Command::new("VBoxManage")
            .args(&["import", &vbox_path])
            .status()?;
            
        if !status.success() {
            return Err(DeploymentError::VirtualMachineError("导入VirtualBox虚拟机失败".to_string()));
        }
        
        // 启动虚拟机
        self.log("启动虚拟机...");
        let status = Command::new("VBoxManage")
            .args(&["startvm", "Rust AGI OS GUI"])
            .status()?;
            
        if !status.success() {
            return Err(DeploymentError::VirtualMachineError("启动VirtualBox虚拟机失败".to_string()));
        }
        
        // 等待虚拟机启动
        std::thread::sleep(std::time::Duration::from_secs(30));
        
        // 复制文件到虚拟机
        self.log("复制文件到虚拟机...");
        let status = Command::new("VBoxManage")
            .args(&[
                "guestcontrol",
                "Rust AGI OS GUI",
                "copyto",
                "--target-directory", "/opt/rust_agi_os",
                package_dir,
            ])
            .status()?;
            
        if !status.success() {
            return Err(DeploymentError::VirtualMachineError("复制文件到VirtualBox虚拟机失败".to_string()));
        }
        
        // 在虚拟机中运行启动脚本
        self.log("在虚拟机中运行启动脚本...");
        let status = Command::new("VBoxManage")
            .args(&[
                "guestcontrol",
                "Rust AGI OS GUI",
                "run",
                "--exe", "/bin/bash",
                "--",
                "/opt/rust_agi_os/start.sh",
            ])
            .status()?;
            
        if !status.success() {
            return Err(DeploymentError::VirtualMachineError("在VirtualBox虚拟机中运行启动脚本失败".to_string()));
        }
        
        self.log("VirtualBox虚拟机部署完成");
        
        Ok(())
    }
    
    /// 部署到QEMU
    fn deploy_to_qemu(&mut self, package_dir: &str) -> Result<(), DeploymentError> {
        self.log("部署到QEMU虚拟机...");
        
        // 检查QEMU工具是否可用
        let qemu_status = Command::new("which")
            .arg("qemu-system-x86_64")
            .status();
            
        if qemu_status.is_err() || !qemu_status.unwrap().success() {
            self.log("警告: QEMU工具(qemu-system-x86_64)不可用，无法自动部署");
            return Ok(());
        }
        
        // 运行QEMU启动脚本
        self.log("运行QEMU启动脚本...");
        let qemu_script = format!("{}/vm/start_qemu.sh", self.config.output_dir);
        let status = Command::new("bash")
            .arg(&qemu_script)
            .status()?;
            
        if !status.success() {
            return Err(DeploymentError::VirtualMachineError("运行QEMU启动脚本失败".to_string()));
        }
        
        self.log("QEMU虚拟机部署完成");
        
        Ok(())
    }
    
    /// 部署到Hyper-V
    fn deploy_to_hyperv(&mut self, package_dir: &str) -> Result<(), DeploymentError> {
        self.log("部署到Hyper-V虚拟机...");
        
        // 检查是否在Windows环境
        if !cfg!(target_os = "windows") {
            self.log("警告: 不是Windows环境，无法部署到Hyper-V");
            return Ok(());
        }
        
        // 检查PowerShell是否可用
        let powershell_status = Command::new("which")
            .arg("powershell")
            .status();
            
        if powershell_status.is_err() || !powershell_status.unwrap().success() {
            self.log("警告: PowerShell不可用，无法自动部署到Hyper-V");
            return Ok(());
        }
        
        // 运行Hyper-V创建脚本
        self.log("运行Hyper-V创建脚本...");
        let hyperv_script = format!("{}/vm/create_hyperv.ps1", self.config.output_dir);
        let status = Command::new("powershell")
            .args(&["-ExecutionPolicy", "Bypass", "-File", &hyperv_script])
            .status()?;
            
        if !status.success() {
            return Err(DeploymentError::VirtualMachineError("运行Hyper-V创建脚本失败".to_string()));
        }
        
        self.log("Hyper-V虚拟机部署完成");
        
        Ok(())
    }
    
    /// 生成部署文档
    pub fn generate_deployment_documentation(&mut self) -> Result<String, DeploymentError> {
        self.log("生成部署文档...");
        
        let doc_path = format!("{}/deployment_guide.md", self.config.output_dir);
        let mut file = File::create(&doc_path)?;
        
        writeln!(file, "# Rust AGI OS GUI 部署指南")?;
        writeln!(file)?;
        writeln!(file, "## 系统要求")?;
        writeln!(file)?;
        writeln!(file, "- 操作系统: Linux (推荐 Ubuntu 20.04 或更高版本)")?;
        writeln!(file, "- CPU: 4核心或更多")?;
        writeln!(file, "- 内存: 4GB或更多")?;
        writeln!(file, "- 显卡: 支持OpenGL 3.3或更高版本")?;
        
        if self.config.enable_4k_resolution {
            writeln!(file, "- 显示器: 支持4K分辨率(3840x2160)")?;
        } else {
            writeln!(file, "- 显示器: 支持1080p分辨率(1920x1080)或更高")?;
        }
        
        writeln!(file)?;
        writeln!(file, "## 虚拟机环境")?;
        writeln!(file)?;
        writeln!(file, "本应用已配置为在{}虚拟机中运行。", self.config.vm_type.as_str())?;
        writeln!(file)?;
        
        match self.config.vm_type {
            VirtualMachineType::VMware => {
                writeln!(file, "### VMware配置")?;
                writeln!(file)?;
                writeln!(file, "1. 打开VMware Workstation/Fusion")?;
                writeln!(file, "2. 选择\"打开虚拟机\"")?;
                writeln!(file, "3. 导航到`{}/vm/rust_agi_os.vmx`", self.config.output_dir)?;
                writeln!(file, "4. 启动虚拟机")?;
                writeln!(file, "5. 登录后，运行以下命令启动GUI桌面环境:")?;
                writeln!(file, "   ```")?;
                writeln!(file, "   cd /opt/rust_agi_os")?;
                writeln!(file, "   ./start.sh")?;
                writeln!(file, "   ```")?;
            },
            VirtualMachineType::VirtualBox => {
                writeln!(file, "### VirtualBox配置")?;
                writeln!(file)?;
                writeln!(file, "1. 打开VirtualBox")?;
                writeln!(file, "2. 选择\"导入设备\"")?;
                writeln!(file, "3. 导航到`{}/vm/rust_agi_os.vbox`", self.config.output_dir)?;
                writeln!(file, "4. 完成导入后启动虚拟机")?;
                writeln!(file, "5. 登录后，运行以下命令启动GUI桌面环境:")?;
                writeln!(file, "   ```")?;
                writeln!(file, "   cd /opt/rust_agi_os")?;
                writeln!(file, "   ./start.sh")?;
                writeln!(file, "   ```")?;
            },
            VirtualMachineType::QEMU => {
                writeln!(file, "### QEMU配置")?;
                writeln!(file)?;
                writeln!(file, "1. 确保已安装QEMU")?;
                writeln!(file, "2. 运行以下命令启动虚拟机:")?;
                writeln!(file, "   ```")?;
                writeln!(file, "   cd {}/vm", self.config.output_dir)?;
                writeln!(file, "   ./start_qemu.sh")?;
                writeln!(file, "   ```")?;
                writeln!(file, "3. 登录后，运行以下命令启动GUI桌面环境:")?;
                writeln!(file, "   ```")?;
                writeln!(file, "   cd /opt/rust_agi_os")?;
                writeln!(file, "   ./start.sh")?;
                writeln!(file, "   ```")?;
            },
            VirtualMachineType::HyperV => {
                writeln!(file, "### Hyper-V配置")?;
                writeln!(file)?;
                writeln!(file, "1. 以管理员身份打开PowerShell")?;
                writeln!(file, "2. 运行以下命令创建虚拟机:")?;
                writeln!(file, "   ```")?;
                writeln!(file, "   cd {}/vm", self.config.output_dir)?;
                writeln!(file, "   ./create_hyperv.ps1")?;
                writeln!(file, "   ```")?;
                writeln!(file, "3. 在Hyper-V管理器中启动虚拟机")?;
                writeln!(file, "4. 登录后，运行以下命令启动GUI桌面环境:")?;
                writeln!(file, "   ```")?;
                writeln!(file, "   cd /opt/rust_agi_os")?;
                writeln!(file, "   ./start.sh")?;
                writeln!(file, "   ```")?;
            },
        }
        
        writeln!(file)?;
        writeln!(file, "## 直接安装")?;
        writeln!(file)?;
        writeln!(file, "如果您希望直接在物理机上安装，请按照以下步骤操作:")?;
        writeln!(file)?;
        writeln!(file, "1. 将整个`{}`目录复制到目标机器", self.config.output_dir)?;
        writeln!(file, "2. 确保目标机器满足系统要求")?;
        writeln!(file, "3. 运行以下命令启动GUI桌面环境:")?;
        writeln!(file, "   ```")?;
        writeln!(file, "   cd {}", self.config.output_dir)?;
        writeln!(file, "   ./start.sh")?;
        writeln!(file, "   ```")?;
        
        writeln!(file)?;
        writeln!(file, "## 故障排除")?;
        writeln!(file)?;
        writeln!(file, "### 显示问题")?;
        writeln!(file)?;
        writeln!(file, "如果遇到显示问题，请尝试以下解决方案:")?;
        writeln!(file)?;
        writeln!(file, "1. 确保虚拟机已启用3D加速")?;
        writeln!(file, "2. 更新显卡驱动程序")?;
        writeln!(file, "3. 调整分辨率设置:")?;
        writeln!(file, "   ```")?;
        writeln!(file, "   export RUST_AGI_OS_RESOLUTION=1920x1080  # 降低分辨率")?;
        writeln!(file, "   ./start.sh")?;
        writeln!(file, "   ```")?;
        
        writeln!(file)?;
        writeln!(file, "### 输入问题")?;
        writeln!(file)?;
        writeln!(file, "如果遇到键盘或鼠标输入问题，请尝试以下解决方案:")?;
        writeln!(file)?;
        writeln!(file, "1. 确保虚拟机已启用USB控制器")?;
        writeln!(file, "2. 在虚拟机设置中启用输入设备直通")?;
        writeln!(file, "3. 尝试使用不同的输入设备")?;
        
        writeln!(file)?;
        writeln!(file, "## 联系支持")?;
        writeln!(file)?;
        writeln!(file, "如果您遇到任何问题，请联系我们的支持团队。")?;
        
        self.log(&format!("部署文档已生成: {}", doc_path));
        
        Ok(doc_path)
    }
}

/// 创建部署配置
pub fn create_deployment_config(output_dir: &str, vm_type: VirtualMachineType) -> DeploymentConfig {
    DeploymentConfig::new(output_dir, vm_type)
}

/// 创建部署管理器
pub fn create_deployment_manager(config: DeploymentConfig) -> DeploymentManager {
    DeploymentManager::new(config)
}

/// 打包并部署到虚拟机
pub fn package_and_deploy(
    source_dir: &str,
    output_dir: &str,
    vm_type: VirtualMachineType,
    vm_disk_path: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    // 创建部署配置
    let mut config = create_deployment_config(output_dir, vm_type);
    
    // 设置虚拟机磁盘路径
    if let Some(disk_path) = vm_disk_path {
        config.set_vm_disk_path(disk_path);
    }
    
    // 创建部署管理器
    let mut manager = create_deployment_manager(config);
    
    // 打包应用
    let package_dir = manager.package_application(source_dir)?;
    
    // 部署到虚拟机
    manager.deploy_to_vm(&package_dir)?;
    
    // 生成部署文档
    manager.generate_deployment_documentation()?;
    
    Ok(())
}
