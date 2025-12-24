use sysinfo::{System};
use owo_colors::OwoColorize;

mod gpu;

fn main() {
    // Please note that we use "new_all" to ensure that all lists of
    // CPUs and processes are filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    println!("{} {}", "OS:".green().bold(), System::long_os_version().unwrap_or_else(|| "<unknown>".to_owned()));
    println!("{} {}", "CPU:".green().bold(), sys.cpus()[0].brand());
    println!("{} {}", "GPU:".green().bold(), gpu::get_gpu_name().unwrap());
    println!("{} {}B / {}B", "Memory:".green().bold(), sys.used_memory(), sys.total_memory());
}
