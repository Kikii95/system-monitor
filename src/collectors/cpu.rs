use anyhow::Result;
use sysinfo::System;

use super::History60;

#[derive(Clone, Default)]
pub struct CpuData {
    pub model: String,
    pub physical_cores: usize,
    pub logical_cores: usize,
    pub global_usage: f32,
    pub per_core_usage: Vec<f32>,
    pub frequency_mhz: u64,
}

pub struct CpuCollector {
    system: System,
    pub data: CpuData,
    pub history: History60,
}

impl CpuCollector {
    pub fn new() -> Result<Self> {
        let mut system = System::new_all();
        system.refresh_cpu_all();

        // Wait a bit for accurate first reading
        std::thread::sleep(std::time::Duration::from_millis(200));
        system.refresh_cpu_all();

        let cpus = system.cpus();
        let physical_cores = system.physical_core_count().unwrap_or(cpus.len());

        let data = CpuData {
            model: cpus.first().map(|c| c.brand().to_string()).unwrap_or_default(),
            physical_cores,
            logical_cores: cpus.len(),
            global_usage: 0.0,
            per_core_usage: vec![0.0; cpus.len()],
            frequency_mhz: cpus.first().map(|c| c.frequency()).unwrap_or(0),
        };

        Ok(Self {
            system,
            data,
            history: History60::default(),
        })
    }

    pub fn collect(&mut self) -> Result<()> {
        self.system.refresh_cpu_all();

        let cpus = self.system.cpus();

        // Calculate global usage (average of all cores)
        let total: f32 = cpus.iter().map(|c| c.cpu_usage()).sum();
        self.data.global_usage = total / cpus.len() as f32;

        // Per-core usage
        self.data.per_core_usage = cpus.iter().map(|c| c.cpu_usage()).collect();

        // Current frequency
        self.data.frequency_mhz = cpus.first().map(|c| c.frequency()).unwrap_or(0);

        // Update history
        self.history.push(self.data.global_usage);

        Ok(())
    }
}
