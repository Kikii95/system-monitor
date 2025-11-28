use anyhow::Result;
use sysinfo::{System, ProcessesToUpdate, ProcessStatus};

#[derive(Clone, Default)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub memory_bytes: u64,
    pub memory_percent: f32,
    pub threads: usize,
    pub status: String,
}

#[derive(Clone, Default)]
pub struct ProcessData {
    pub total_processes: usize,
    pub total_threads: usize,
    pub running: usize,
    pub sleeping: usize,
    pub zombies: usize,
    pub top_cpu: Vec<ProcessInfo>,
    pub top_memory: Vec<ProcessInfo>,
}

pub struct ProcessCollector {
    system: System,
    pub data: ProcessData,
    total_memory: u64,
}

impl ProcessCollector {
    pub fn new() -> Result<Self> {
        let mut system = System::new_all();
        system.refresh_all();
        let total_memory = system.total_memory();

        Ok(Self {
            system,
            data: ProcessData::default(),
            total_memory,
        })
    }

    pub fn collect(&mut self) -> Result<()> {
        self.system.refresh_processes(ProcessesToUpdate::All, true);

        let processes: Vec<_> = self.system.processes().values().collect();

        // Count process states
        self.data.total_processes = processes.len();
        self.data.total_threads = 0;
        self.data.running = 0;
        self.data.sleeping = 0;
        self.data.zombies = 0;

        let mut process_list: Vec<ProcessInfo> = Vec::with_capacity(processes.len());

        for proc in &processes {
            // Count by status
            match proc.status() {
                ProcessStatus::Run => self.data.running += 1,
                ProcessStatus::Sleep => self.data.sleeping += 1,
                ProcessStatus::Zombie => self.data.zombies += 1,
                _ => {}
            }

            let threads = proc.tasks().map(|t| t.len()).unwrap_or(1);
            self.data.total_threads += threads;

            let memory = proc.memory();
            let memory_percent = if self.total_memory > 0 {
                (memory as f32 / self.total_memory as f32) * 100.0
            } else {
                0.0
            };

            process_list.push(ProcessInfo {
                pid: proc.pid().as_u32(),
                name: proc.name().to_string_lossy().to_string(),
                cpu_percent: proc.cpu_usage(),
                memory_bytes: memory,
                memory_percent,
                threads,
                status: format!("{:?}", proc.status()),
            });
        }

        // Sort by CPU and get top 5
        process_list.sort_by(|a, b| b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap());
        self.data.top_cpu = process_list.iter().take(5).cloned().collect();

        // Sort by memory and get top 5
        process_list.sort_by(|a, b| b.memory_bytes.cmp(&a.memory_bytes));
        self.data.top_memory = process_list.iter().take(5).cloned().collect();

        Ok(())
    }
}

/// Format memory for processes
pub fn format_proc_memory(bytes: u64) -> String {
    const MB: u64 = 1024 * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1}G", bytes as f64 / GB as f64)
    } else {
        format!("{}M", bytes / MB)
    }
}
