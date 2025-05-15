use sysinfo::System;

pub fn info(){
    let s = System::new_all();
    println!("Total: {} GB", s.total_memory()/1000000000);
    println!("Available: {} GB", s.available_memory()/1000000000);
    println!("Total Swap: {} GB", s.total_swap()/1000000000);
    println!("Used Swap: {} GB", s.used_swap()/1000000000);
}