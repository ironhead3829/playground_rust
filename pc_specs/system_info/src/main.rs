use sysinfo::{System, Disks, Networks};

fn main() {
    // Keep a mutable instance to refresh data
    let mut sys = System::new_all();
    
    // CRITICAL: Modern sysinfo requires explicitly calling refresh on your instance 
    // to populate the actual metrics (CPU, Memory, Disks, Networks, etc.)
    sys.refresh_all();
    
    println!("=== Computer Specifications ===\n");
    
    // CPU Information
    println!("🖥️  SYSTEM INFORMATION:");
    println!("---------------------");
    
    // Call static/associated functions directly on the type using ::
    if let Some(platform) = System::name() {
        println!("Platform:        {}", platform);
    }
    if let Some(kernel) = System::kernel_version() {
        println!("Kernel Version:  {}", kernel);
    }
    
    // Frequency checks are supported across modern systems implicitly
    println!("Total CPU Cores: {}", sys.cpus().len());
    for (i, cpu) in sys.cpus().iter().enumerate() {
        println!("  Core {}: {} MHz", i + 1, cpu.frequency());
    }
    
    // Memory Information
    println!("");
    println!("📊 MEMORY INFORMATION:");
    println!("----------------------");
    if sys.total_memory() > 0 {
        // Modern sysinfo returns raw Bytes (B). 
        // We divide by 1024 to convert it back to Kilobytes (KB) to keep your formatting!
        println!("Total memory: {} KB", sys.total_memory() / 1024);
        println!("Used memory:  {} KB", sys.used_memory() / 1024);
    }        
    
    // Disk Information
    println!("");
    println!("💾 DISK INFORMATION:");
    println!("--------------------");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("\nDisk Name:   {:?}", disk.name());
        println!("  File System:{:?}", disk.file_system());
        println!("  Mount Point:{:?}", disk.mount_point());
        // Capacity logic remains unchanged
        println!("  Capacity:   {} MB", disk.total_space() / (1024 * 1024));
    }
    
    // Network Information
    println!("");
    println!("🌐 NETWORK INFORMATION:");
    println!("----------------------");
    let networks = Networks::new_with_refreshed_list();
    for (name, network_data) in &networks {
        println!("\nNetwork Interface: {}", name);
        println!("  Data Received:    {} bytes", network_data.total_received());
        println!("  Data Transmitted: {} bytes", network_data.total_transmitted());
    }
    
    println!("\n=== End of System Info ===");
}
