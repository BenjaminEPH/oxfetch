pub mod cpu;
pub mod kernel;
pub mod memory;
pub mod os;
pub mod shell;
pub mod uptime;

pub trait InfoSource {
    fn label(&self) -> &str;
    fn value(&self) -> String;
    fn enabled(&self) -> bool {
        true
    }
}
pub struct SysInfo {
    pub fields: Vec<Box<dyn InfoSource>>,
}

impl SysInfo {
    pub fn collect() -> Self {
        Self {
            fields: vec![
                Box::new(os::OsInfo::new()),
                Box::new(kernel::KernelInfo::new()),
                Box::new(cpu::CpuInfo::new()),
                Box::new(memory::MemoryInfo::new()),
                Box::new(uptime::UptimeInfo::new()),
                Box::new(shell::ShellInfo::new()),
            ],
        }
    }
}
