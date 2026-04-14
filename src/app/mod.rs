use sysinfo::System;

pub struct App {
    pub system: System,
    pub cpu_usage: Vec<f32>,
    pub memory_used: u64,
    pub memory_total: u64,
}

impl App {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self { cpu_usage: vec![], memory_total: 0, memory_used: 0, system }
    }

    pub fn update(&mut self) {
        self.system.refresh_cpu_all();
        self.system.refresh_memory();
        self.cpu_usage = self.system.cpus().iter().map_while(|c| Some(c.cpu_usage())).collect();
        self.memory_used = self.system.used_memory();
        self.memory_total = self.system.total_memory();
    }
}
