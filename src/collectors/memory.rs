use anyhow::Result;
use sysinfo::System;

use super::History60;

#[derive(Clone, Default)]
pub struct MemoryData {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f32,
    pub swap_total: u64,
    pub swap_used: u64,
    pub swap_percent: f32,
}

pub struct MemoryCollector {
    system: System,
    pub data: MemoryData,
    pub history: History60,
}

impl MemoryCollector {
    pub fn new() -> Result<Self> {
        let mut system = System::new_all();
        system.refresh_memory();

        Ok(Self {
            system,
            data: MemoryData::default(),
            history: History60::default(),
        })
    }

    pub fn collect(&mut self) -> Result<()> {
        self.system.refresh_memory();

        self.data.total = self.system.total_memory();
        self.data.used = self.system.used_memory();
        self.data.available = self.system.available_memory();

        self.data.usage_percent = if self.data.total > 0 {
            (self.data.used as f32 / self.data.total as f32) * 100.0
        } else {
            0.0
        };

        self.data.swap_total = self.system.total_swap();
        self.data.swap_used = self.system.used_swap();
        self.data.swap_percent = if self.data.swap_total > 0 {
            (self.data.swap_used as f32 / self.data.swap_total as f32) * 100.0
        } else {
            0.0
        };

        // Update history
        self.history.push(self.data.usage_percent);

        Ok(())
    }
}

/// Format bytes to human readable string
pub fn format_bytes(bytes: u64) -> String {
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else {
        format!("{:.0} MB", bytes as f64 / MB as f64)
    }
}
