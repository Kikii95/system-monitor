use anyhow::Result;
use sysinfo::Disks;
use std::time::Instant;
use std::fs;

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
    pub reads_total: u64,   // total bytes read
    pub writes_total: u64,  // total bytes written
}

pub struct DiskCollector {
    disks: Disks,
    pub data: DiskData,
    pub read_history: History60,
    pub write_history: History60,
    last_update: Instant,
    device_name: String,
    last_sectors_read: u64,
    last_sectors_written: u64,
}

impl DiskCollector {
    pub fn new() -> Result<Self> {
        let disks = Disks::new_with_refreshed_list();

        // Find primary disk (root mount or largest)
        let primary = disks
            .iter()
            .find(|d| d.mount_point().to_string_lossy() == "/")
            .or_else(|| disks.iter().max_by_key(|d| d.total_space()));

        let (data, device_name) = if let Some(disk) = primary {
            let total = disk.total_space();
            let free = disk.available_space();
            let used = total.saturating_sub(free);

            // Extract device name (e.g., "sda" from "/dev/sda1")
            let dev_name = disk.name().to_string_lossy().to_string();
            let device = extract_device_name(&dev_name);

            (DiskData {
                name: dev_name,
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
                reads_total: 0,
                writes_total: 0,
            }, device)
        } else {
            (DiskData::default(), String::new())
        };

        // Get initial disk stats
        let (sectors_read, sectors_written) = read_disk_stats(&device_name).unwrap_or((0, 0));

        Ok(Self {
            disks,
            data,
            read_history: History60::default(),
            write_history: History60::default(),
            last_update: Instant::now(),
            device_name,
            last_sectors_read: sectors_read,
            last_sectors_written: sectors_written,
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

        // Read I/O stats from /proc/diskstats
        if !self.device_name.is_empty() {
            if let Some((sectors_read, sectors_written)) = read_disk_stats(&self.device_name) {
                // Calculate bytes diff (1 sector = 512 bytes)
                let bytes_read = (sectors_read.saturating_sub(self.last_sectors_read)) * 512;
                let bytes_written = (sectors_written.saturating_sub(self.last_sectors_written)) * 512;

                // Calculate speed (bytes per second)
                self.data.read_speed = bytes_read as f64 / elapsed;
                self.data.write_speed = bytes_written as f64 / elapsed;

                // Update totals
                self.data.reads_total = sectors_read * 512;
                self.data.writes_total = sectors_written * 512;

                // Save for next iteration
                self.last_sectors_read = sectors_read;
                self.last_sectors_written = sectors_written;
            }
        }

        self.last_update = Instant::now();

        // Update history (in MB/s)
        self.read_history.push((self.data.read_speed / 1_000_000.0) as f32);
        self.write_history.push((self.data.write_speed / 1_000_000.0) as f32);

        Ok(())
    }
}

/// Extract base device name (e.g., "sda" from "/dev/sda1" or "sda1")
fn extract_device_name(name: &str) -> String {
    let name = name.trim_start_matches("/dev/");
    // Remove partition number (e.g., "sda1" -> "sda", "nvme0n1p1" -> "nvme0n1")
    if name.starts_with("nvme") {
        // NVMe format: nvme0n1p1 -> nvme0n1
        if let Some(pos) = name.rfind('p') {
            if name[pos + 1..].chars().all(|c| c.is_ascii_digit()) {
                return name[..pos].to_string();
            }
        }
        name.to_string()
    } else {
        // Standard format: sda1 -> sda
        name.trim_end_matches(|c: char| c.is_ascii_digit()).to_string()
    }
}

/// Read disk stats from /proc/diskstats
/// Returns (sectors_read, sectors_written) for the given device
fn read_disk_stats(device: &str) -> Option<(u64, u64)> {
    let content = fs::read_to_string("/proc/diskstats").ok()?;

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 11 && parts[2] == device {
            // Field 6 (index 5): sectors read
            // Field 10 (index 9): sectors written
            let sectors_read = parts[5].parse().ok()?;
            let sectors_written = parts[9].parse().ok()?;
            return Some((sectors_read, sectors_written));
        }
    }

    None
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
