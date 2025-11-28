use anyhow::Result;

use super::History60;

#[derive(Clone, Default)]
pub struct GpuData {
    pub available: bool,
    pub name: String,
    pub driver_version: String,
    pub usage_percent: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub memory_percent: f32,
    pub temperature: u32,
    pub fan_speed: u32,
    pub power_draw: u32,
    pub power_limit: u32,
    pub clock_core: u32,
    pub clock_memory: u32,
    pub encoder_usage: u32,
    pub decoder_usage: u32,
    pub pcie_gen: u32,
    pub pcie_width: u32,
}

pub struct GpuCollector {
    #[cfg(feature = "gpu")]
    nvml: Option<nvml_wrapper::Nvml>,
    #[cfg(feature = "gpu")]
    device: Option<nvml_wrapper::Device<'static>>,
    pub data: GpuData,
    pub history: History60,
}

impl GpuCollector {
    pub fn new() -> Result<Self> {
        #[cfg(feature = "gpu")]
        {
            match nvml_wrapper::Nvml::init() {
                Ok(nvml) => {
                    // Leak nvml to get 'static lifetime (acceptable for single instance)
                    let nvml = Box::leak(Box::new(nvml));

                    match nvml.device_by_index(0) {
                        Ok(device) => {
                            let name = device.name().unwrap_or_default();
                            let driver = nvml.sys_driver_version().unwrap_or_default();

                            let device = Box::leak(Box::new(device));

                            return Ok(Self {
                                nvml: Some(unsafe { std::ptr::read(nvml) }),
                                device: Some(unsafe { std::ptr::read(device) }),
                                data: GpuData {
                                    available: true,
                                    name,
                                    driver_version: driver,
                                    ..Default::default()
                                },
                                history: History60::default(),
                            });
                        }
                        Err(_) => {}
                    }
                }
                Err(_) => {}
            }
        }

        // No GPU available
        Ok(Self {
            #[cfg(feature = "gpu")]
            nvml: None,
            #[cfg(feature = "gpu")]
            device: None,
            data: GpuData::default(),
            history: History60::default(),
        })
    }

    pub fn collect(&mut self) -> Result<()> {
        #[cfg(feature = "gpu")]
        if let Some(ref device) = self.device {
            // Usage
            if let Ok(util) = device.utilization_rates() {
                self.data.usage_percent = util.gpu as f32;
                self.data.encoder_usage = util.memory;
            }

            // Memory
            if let Ok(mem) = device.memory_info() {
                self.data.memory_used = mem.used;
                self.data.memory_total = mem.total;
                self.data.memory_percent = (mem.used as f32 / mem.total as f32) * 100.0;
            }

            // Temperature
            if let Ok(temp) = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu) {
                self.data.temperature = temp;
            }

            // Fan speed
            if let Ok(fan) = device.fan_speed(0) {
                self.data.fan_speed = fan;
            }

            // Power
            if let Ok(power) = device.power_usage() {
                self.data.power_draw = power / 1000; // mW to W
            }
            if let Ok(limit) = device.power_management_limit() {
                self.data.power_limit = limit / 1000;
            }

            // Clocks
            if let Ok(clock) = device.clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics) {
                self.data.clock_core = clock;
            }
            if let Ok(clock) = device.clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory) {
                self.data.clock_memory = clock;
            }

            // PCIe
            if let Ok(gen) = device.current_pcie_link_gen() {
                self.data.pcie_gen = gen;
            }
            if let Ok(width) = device.current_pcie_link_width() {
                self.data.pcie_width = width;
            }

            // Encoder/Decoder
            if let Ok(enc) = device.encoder_utilization() {
                self.data.encoder_usage = enc.utilization;
            }
            if let Ok(dec) = device.decoder_utilization() {
                self.data.decoder_usage = dec.utilization;
            }

            self.history.push(self.data.usage_percent);
        }

        Ok(())
    }

    pub fn is_available(&self) -> bool {
        self.data.available
    }
}
