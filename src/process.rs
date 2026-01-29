use std::collections::HashSet;
use sysinfo::{Pid, Process, System};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessCategory {
    System,  // 0 - Blocked
    Service, // 1 - Confirmation
    User,    // 2 - Free
}

impl ProcessCategory {
    pub fn as_str(&self) -> &str {
        match self {
            ProcessCategory::System => "System",
            ProcessCategory::Service => "Service",
            ProcessCategory::User => "User",
        }
    }

    pub fn color_code(&self) -> u8 {
        match self {
            ProcessCategory::System => 1,  // Red
            ProcessCategory::Service => 3, // Yellow
            ProcessCategory::User => 2,    // Green
        }
    }
}

pub struct Categorizer {
    system_whitelist: HashSet<String>,
}

impl Categorizer {
    pub fn new() -> Self {
        let mut whitelist = HashSet::new();
        // Common critical Windows processes (and some generic ones)
        let system_procs = vec![
            "System",
            "Registry",
            "smss.exe",
            "csrss.exe",
            "wininit.exe",
            "services.exe",
            "lsass.exe",
            "svchost.exe",
            "fontdrvhost.exe",
            "Memory Compression",
            "spoolsv.exe",
            "winlogon.exe",
            "dwm.exe",
        ];

        for name in system_procs {
            whitelist.insert(name.to_lowercase());
        }

        Self {
            system_whitelist: whitelist,
        }
    }

    pub fn categorize(&self, process: &Process) -> ProcessCategory {
        let name = process.name().to_string_lossy().to_lowercase();

        // Simple heuristic for Windows
        if self.system_whitelist.contains(&name) {
            return ProcessCategory::System;
        }

        // Usually low PIDs on Windows are system, but names are more reliable for TUI context.
        // Pid 0 is Idle, Pid 4 is System usually.
        let pid = process.pid().to_string().parse::<u32>().unwrap_or(0);
        if pid <= 4 {
            return ProcessCategory::System;
        }

        // If it's running from Common Files or Windows directory, likely a service/system component
        // This is a naive check, but serves the purpose for now.
        if let Some(exe) = process.exe() {
            let path_str = exe.to_string_lossy().to_lowercase();
            if path_str.contains("\\windows\\system32\\") {
                return ProcessCategory::Service;
            }
        }

        ProcessCategory::User
    }
}

#[derive(Debug, Clone)]
pub struct GlobalStats {
    pub total_memory: u64,
    pub used_memory: u64,
    pub cpu_usage: f32,
    pub uptime: u64,
}

pub struct ProcessData {
    pub pid: u32,
    pub name: String,
    pub memory: u64,    // bytes
    pub cpu_usage: f32, // percentage
    pub category: ProcessCategory,
}

pub fn collect_processes(
    system: &mut System,
    categorizer: &Categorizer,
) -> (Vec<ProcessData>, GlobalStats) {
    system.refresh_all();
    let mut processes = Vec::new();

    for (pid, process) in system.processes() {
        let category = categorizer.categorize(process);

        processes.push(ProcessData {
            pid: pid.to_string().parse::<u32>().unwrap_or(0),
            name: process.name().to_string_lossy().to_string(),
            memory: process.memory(),
            cpu_usage: process.cpu_usage(),
            category,
        });
    }

    // Sort by CPU usage desc
    processes.sort_by(|a, b| {
        b.cpu_usage
            .partial_cmp(&a.cpu_usage)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let global_cpu: f32 = if system.cpus().is_empty() {
        0.0
    } else {
        system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / system.cpus().len() as f32
    };

    let stats = GlobalStats {
        total_memory: system.total_memory(),
        used_memory: system.used_memory(),
        cpu_usage: global_cpu,
        uptime: System::uptime(),
    };

    (processes, stats)
}
