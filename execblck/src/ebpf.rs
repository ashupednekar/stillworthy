use shared::prelude::Result;
mod syscall_counter {
    include!(concat!(env!("OUT_DIR"), "/syscall_counter.skel.rs"));
}
use std::time::Duration;
use syscall_counter::*;

pub struct ExtensionEbpf {}

impl ExtensionEbpf {
    pub fn new() -> Result<Self> {
        return Ok(ExtensionEbpf {});
    }
}

impl crate::Guard for ExtensionEbpf {
    fn notify(&self) -> Result<()> {
        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, std::sync::atomic::Ordering::SeqCst);
        })?;

        // Build and load the BPF program
        let builder = SyscallCounterSkelBuilder::default();
        let open_skel = builder.open()?;
        let mut skel = open_skel.load()?;

        // Attach the BPF program to the kernel tracepoint
        if let Err(e) = skel.attach() {
            eprintln!("Failed to attach BPF program: {}", e);
        }

        println!("Successfully loaded and attached BPF program");
        println!("Tracing syscalls... Press Ctrl+C to stop");

        // Polling loop - print syscall counts periodically
        while running.load(std::sync::atomic::Ordering::SeqCst) {
            std::thread::sleep(Duration::from_secs(5));

            // Access the BPF map
            let syscall_counts = skel.maps().syscall_counts();

            // Print current syscall counts
            println!("Current syscall counts:");
            for entry in syscall_counts.iter() {
                if let Ok((pid, count)) = entry {
                    let pid_u32 = u32::from_ne_bytes(pid.try_into()?);
                    let count_u64 = u64::from_ne_bytes(count.try_into()?);

                    // Try to get process name
                    let proc_name = match std::fs::read_to_string(format!("/proc/{}/comm", pid_u32))
                    {
                        Ok(name) => name.trim().to_string(),
                        Err(_) => "unknown".to_string(),
                    };

                    println!(
                        "PID: {} ({}) - Syscall count: {}",
                        pid_u32, proc_name, count_u64
                    );
                } else {
                    eprintln!("Failed to parse map entry");
                }
            }
        }

        println!("Exiting...");
        Ok(())
    }
}
