use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
    // First we update all information of our `System` struct.
    sys.refresh_all();
    
    // Fetch memory usage
    let total_memory = sys.total_memory(); // in bytes
    let used_memory = sys.used_memory();  // in bytes
    let free_memory = sys.free_memory();

    // // Byte -> KB conversion
    // let total_memory = total_memory/1024;
    // let used_memory = used_memory/1024;
    // let free_memory = free_memory/1024;

    // Print results
    println!("Total Memory: {} MiB", total_memory/1024/1024);
    println!("Used Memory: {} MiB", used_memory/1024/1024);
    println!("Free Memory: {} MiB", free_memory/1024/1024);

}