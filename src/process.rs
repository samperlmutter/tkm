use sysinfo::Process as SIProcess;

#[derive(Clone)]
pub struct Process {
    pub pid: i32,
    pub name: String,
    pub cpu: f32,
    pub mem: u64
}

impl Process {
    pub fn new(si_process: &SIProcess) -> Process {
        Process {
            pid: si_process.pid,
            name: si_process.name.clone(),
            cpu: si_process.cpu_usage,
            mem: si_process.memory
        }
    }

    pub fn format(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0)
        ]
    }
}
