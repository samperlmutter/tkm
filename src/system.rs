extern crate sysinfo;
extern crate sys_info;
extern crate itertools;
extern crate pretty_bytes;

use std::collections::HashMap;
use sysinfo::{SystemExt, ProcessorExt, Process};
use pretty_bytes::converter::convert;
use itertools::Itertools;

pub struct System {
    system: sysinfo::System,
    pub cpu_usage_history: Vec<u64>,
    pub cpu_current_usage: u64,
    pub cpu_num_cores: usize,
    pub mem_total: u64,
    pub mem_free: u64,
    pub mem_used: u64,
    pub mem_usage_history: Vec<u64>,
    pub cpu_core_usages: Vec<u16>,
    pub process_list: HashMap<i32, Process>
}

impl System {
    pub fn new(system: sysinfo::System, initial_size: u16) -> Result<System, failure::Error> {
        let history_width = initial_size / 2;

        // Overall CPU usage
        let cpu_usage_history = vec![0; history_width as usize];
        let cpu_num_cores: usize = system.get_processor_list().len() - 1;

        // Memory usage
        let mem_total = sys_info::mem_info()?.total;
        let mem_usage_history = vec![0; history_width as usize];

        Ok(System {
            system,
            cpu_usage_history,
            cpu_current_usage: 0,
            cpu_num_cores,
            mem_total,
            mem_free: 0,
            mem_used: 0,
            mem_usage_history,
            cpu_core_usages: vec![],
            process_list: HashMap::<i32, Process>::new(),
        })
    }

    pub fn update(&mut self) -> Result<(), failure::Error> {
        self.system.refresh_all();
        let mem_info = sys_info::mem_info()?;

        // Overall CPU usage
        self.cpu_current_usage = (self.system.get_processor_list()[0].get_cpu_usage() * 100.0).round() as u64;
        self.cpu_usage_history.push(self.cpu_current_usage);
        self.cpu_usage_history.remove(0);

        // Memory usage
        self.mem_used = mem_info.total - mem_info.avail;
        self.mem_free = mem_info.avail;
        self.mem_usage_history.push(self.mem_used);
        self.mem_usage_history.remove(0);

        // CPU core usage
        self.cpu_core_usages = self.system.get_processor_list()
            .iter()
            .skip(1)
            .map(|p| (p.get_cpu_usage() * 100.0).round() as u16)
            .collect();

        // Processes
        self.process_list = self.system.get_process_list().clone();

        Ok(())
    }

    pub fn get_processes(&self) -> Vec<Vec<String>> {
        self.process_list.iter()
            .sorted_by(|(_, a), (_, b)| // Sorting by CPU usage
                Ord::cmp(&(a.cpu_usage as u32), &(b.cpu_usage as u32))
            )
            .rev()
            .map(|(pid, process)|
                vec![
                    pid.to_string(),
                    process.name.clone(),
                    format!("{:.2}%", process.cpu_usage),
                    convert((process.memory as f64) * 1000.0)
                ]
            )
            .collect()
    }
}
