use anyhow::Result;
use sysinfo::Networks;
use std::time::Instant;

use super::History60;

#[derive(Clone, Default)]
pub struct NetworkData {
    pub interface: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_speed: f64,      // bytes/sec
    pub tx_speed: f64,      // bytes/sec
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
}

pub struct NetworkCollector {
    networks: Networks,
    pub data: NetworkData,
    pub rx_history: History60,
    pub tx_history: History60,
    last_rx: u64,
    last_tx: u64,
    last_update: Instant,
}

impl NetworkCollector {
    pub fn new() -> Result<Self> {
        let mut networks = Networks::new_with_refreshed_list();
        networks.refresh();

        // Find primary interface (one with most traffic)
        let interface = networks
            .iter()
            .max_by_key(|(_, data)| data.total_received() + data.total_transmitted())
            .map(|(name, _)| name.clone())
            .unwrap_or_else(|| "eth0".to_string());

        let (rx, tx) = networks
            .get(&interface)
            .map(|n| (n.total_received(), n.total_transmitted()))
            .unwrap_or((0, 0));

        Ok(Self {
            networks,
            data: NetworkData {
                interface,
                ..Default::default()
            },
            rx_history: History60::default(),
            tx_history: History60::default(),
            last_rx: rx,
            last_tx: tx,
            last_update: Instant::now(),
        })
    }

    pub fn collect(&mut self) -> Result<()> {
        self.networks.refresh();

        let elapsed = self.last_update.elapsed().as_secs_f64();
        if elapsed < 0.1 {
            return Ok(());
        }

        if let Some(net) = self.networks.get(&self.data.interface) {
            let rx = net.total_received();
            let tx = net.total_transmitted();

            // Calculate speeds
            let rx_diff = rx.saturating_sub(self.last_rx);
            let tx_diff = tx.saturating_sub(self.last_tx);

            self.data.rx_speed = rx_diff as f64 / elapsed;
            self.data.tx_speed = tx_diff as f64 / elapsed;
            self.data.rx_bytes = rx;
            self.data.tx_bytes = tx;
            self.data.rx_packets = net.total_packets_received();
            self.data.tx_packets = net.total_packets_transmitted();
            self.data.rx_errors = net.total_errors_on_received();
            self.data.tx_errors = net.total_errors_on_transmitted();

            self.last_rx = rx;
            self.last_tx = tx;
            self.last_update = Instant::now();

            // Update history (in MB/s for display)
            self.rx_history.push((self.data.rx_speed / 1_000_000.0) as f32);
            self.tx_history.push((self.data.tx_speed / 1_000_000.0) as f32);
        }

        Ok(())
    }
}

/// Format bytes per second to human readable
pub fn format_speed(bytes_per_sec: f64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    if bytes_per_sec >= GB {
        format!("{:.1} GB/s", bytes_per_sec / GB)
    } else if bytes_per_sec >= MB {
        format!("{:.1} MB/s", bytes_per_sec / MB)
    } else if bytes_per_sec >= KB {
        format!("{:.1} KB/s", bytes_per_sec / KB)
    } else {
        format!("{:.0} B/s", bytes_per_sec)
    }
}

/// Format total bytes
pub fn format_total(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.0} MB", bytes as f64 / MB as f64)
    } else {
        format!("{:.0} KB", bytes as f64 / KB as f64)
    }
}
