pub mod cpu;
pub mod memory;
pub mod system;
pub mod gpu;
pub mod network;
pub mod disk;
pub mod process;

pub use cpu::{CpuCollector, CpuData};
pub use memory::{MemoryCollector, MemoryData};
pub use system::{SystemCollector, SystemData};
pub use gpu::{GpuCollector, GpuData};
pub use network::{NetworkCollector, NetworkData};
pub use disk::{DiskCollector, DiskData};
pub use process::{ProcessCollector, ProcessData, ProcessInfo};

use anyhow::Result;
use crate::config::Config;

/// Ring buffer for storing history data (sparklines)
#[derive(Clone)]
pub struct RingBuffer<const N: usize> {
    data: [f32; N],
    head: usize,
    len: usize,
}

impl<const N: usize> Default for RingBuffer<N> {
    fn default() -> Self {
        Self {
            data: [0.0; N],
            head: 0,
            len: 0,
        }
    }
}

impl<const N: usize> RingBuffer<N> {
    pub fn push(&mut self, value: f32) {
        self.data[self.head] = value;
        self.head = (self.head + 1) % N;
        if self.len < N {
            self.len += 1;
        }
    }

    pub fn as_vec(&self) -> Vec<u64> {
        let mut result = Vec::with_capacity(self.len);
        for i in 0..self.len {
            let idx = (self.head + N - self.len + i) % N;
            result.push(self.data[idx] as u64);
        }
        result
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// 60-point history buffer (60 seconds at 1Hz)
pub type History60 = RingBuffer<60>;

/// All system collectors
pub struct Collectors {
    pub cpu: CpuCollector,
    pub memory: MemoryCollector,
    pub system: SystemCollector,
    pub gpu: GpuCollector,
    pub network: NetworkCollector,
    pub disk: DiskCollector,
    pub process: ProcessCollector,
}

impl Collectors {
    pub fn new(config: &Config) -> Result<Self> {
        Ok(Self {
            cpu: CpuCollector::new()?,
            memory: MemoryCollector::new()?,
            system: SystemCollector::new()?,
            gpu: if config.gpu_enabled {
                GpuCollector::new()?
            } else {
                GpuCollector::new()? // Will just report unavailable
            },
            network: NetworkCollector::new()?,
            disk: DiskCollector::new()?,
            process: ProcessCollector::new()?,
        })
    }

    pub fn update(&mut self) -> Result<()> {
        self.cpu.collect()?;
        self.memory.collect()?;
        self.system.collect()?;
        self.gpu.collect()?;
        self.network.collect()?;
        self.disk.collect()?;
        self.process.collect()?;
        Ok(())
    }
}
