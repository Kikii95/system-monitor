use anyhow::Result;
use sysinfo::System;

#[derive(Clone, Default)]
pub struct SystemData {
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub hostname: String,
    pub uptime_secs: u64,
}

pub struct SystemCollector {
    pub data: SystemData,
}

impl SystemCollector {
    pub fn new() -> Result<Self> {
        let data = SystemData {
            os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
            os_version: System::os_version().unwrap_or_else(|| "".to_string()),
            kernel_version: System::kernel_version().unwrap_or_else(|| "".to_string()),
            hostname: System::host_name().unwrap_or_else(|| "localhost".to_string()),
            uptime_secs: System::uptime(),
        };

        Ok(Self { data })
    }

    pub fn collect(&mut self) -> Result<()> {
        self.data.uptime_secs = System::uptime();
        Ok(())
    }
}

/// Format uptime to human readable string
pub fn format_uptime(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, mins)
    } else if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else {
        format!("{}m", mins)
    }
}
