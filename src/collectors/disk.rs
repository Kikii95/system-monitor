use anyhow::Result;
use sysinfo::Disks;
use std::time::Instant;

use super::History60;

#[derive(Clone, Default)]
pub struct DiskData {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub used_space: u64,
    pub free_space: u64,
    pub usage_percent: f32,
    pub read_speed: f64,    // bytes/sec
    pub write_speed: f64,   // bytes/sec
}

pub struct DiskCollector {
    disks: Disks,
    pub data: DiskData,
    pub read_history: History60,
    pub write_history: History60,
    last_update: Instant,
}

impl DiskCollector {
    pub fn new() -> Result<Self> {
        let disks = Disks::new_with_refreshed_list();

        // Find primary disk (root mount or largest)
        let primary = disks
            .iter()
            .find(|d| d.mount_point().to_string_lossy() == "/")
            .or_else(|| disks.iter().max_by_key(|d| d.total_space()));

        let data = if let Some(disk) = primary {
            let total = disk.total_space();
            let free = disk.available_space();
            let used = total.saturating_sub(free);

            DiskData {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total_space: total,
                used_space: used,
                free_space: free,
                usage_percent: if total > 0 {
                    (used as f32 / total as f32) * 100.0
                } else {
                    0.0
                },
                read_speed: 0.0,
                write_speed: 0.0,
            }
        } else {
            DiskData::default()
        };

        Ok(Self {
            disks,
            data,
            read_history: History60::default(),
            write_history: History60::default(),
            last_update: Instant::now(),
        })
    }

    pub fn collect(&mut self) -> Result<()> {
        self.disks.refresh();

        let elapsed = self.last_update.elapsed().as_secs_f64();
        if elapsed < 0.1 {
            return Ok(());
        }

        // Update disk usage
        if let Some(disk) = self.disks
            .iter()
            .find(|d| d.mount_point().to_string_lossy() == self.data.mount_point)
        {
            let total = disk.total_space();
            let free = disk.available_space();
            let used = total.saturating_sub(free);

            self.data.total_space = total;
            self.data.used_space = used;
            self.data.free_space = free;
            self.data.usage_percent = if total > 0 {
                (used as f32 / total as f32) * 100.0
            } else {
                0.0
            };
        }

        // Note: Disk I/O speeds require reading /proc/diskstats
        // For now, we'll use placeholder values
        // TODO: Implement proper I/O monitoring from /proc/diskstats

        self.last_update = Instant::now();

        // Update history
        self.read_history.push((self.data.read_speed / 1_000_000.0) as f32);
        self.write_history.push((self.data.write_speed / 1_000_000.0) as f32);

        Ok(())
    }
}

/// Format disk space
pub fn format_space(bytes: u64) -> String {
    const GB: u64 = 1024 * 1024 * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.1} TB", bytes as f64 / TB as f64)
    } else {
        format!("{:.0} GB", bytes as f64 / GB as f64)
    }
}
