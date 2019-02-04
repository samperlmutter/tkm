extern crate sysinfo;

use sysinfo::{SystemExt, ProcessorExt};

pub struct System<'a> {
    pub cpu_usage_history: Vec<u64>,
    pub cpu_current_usage: u64,
    pub cpu_num_cores: usize,
    system: &'a mut sysinfo::System
}

impl<'a> System<'a> {
    pub fn new(system: &mut sysinfo::System, initial_size: u16) -> System {
        let cpu_usage_history = vec![0; (initial_size / 2) as usize];
        let cpu_current_usage: u64 = 0;
        let cpu_num_cores: usize = system.get_processor_list().len() - 1;
        System {
            cpu_usage_history,
            cpu_current_usage,
            cpu_num_cores,
            system
        }
    }

    pub fn update(&mut self) {
        self.system.refresh_all();
        let cpu_usage = (self.system.get_processor_list()[0].get_cpu_usage() * 100.0).round() as u64;
        self.cpu_current_usage = cpu_usage;
        self.cpu_usage_history.push(self.cpu_current_usage);
        self.cpu_usage_history.remove(0);
    }
}
