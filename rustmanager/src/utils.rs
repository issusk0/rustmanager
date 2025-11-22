use std::*;
use sysinfo::{Pid, System};


pub fn info(){
    let mut sys = System::new_all();
    sys.refresh_all();
    let cpu_usage = sys.global_cpu_usage();
    let core_cpus = sys.cpus().len();
    let total_threads = std::thread::available_parallelism().unwrap();
    let total_memory = sys.total_memory();
    let free_memory = sys.free_memory();
    println!("Total RAM Memory: {} MB", total_memory / 1024 / 1024);
    println!("Free RAM memory: {} MB", free_memory / 1024 / 1024);
    println!("Total CPU usage: {}% ", cpu_usage);
    println!("Total core cpus: {}",core_cpus);
    println!("Total Threads: {}",total_threads);

    for (pid, process) in sys.processes() {
        let name =process.name().to_string_lossy();
        let status =process.status();
        let memory = (process.memory()as f64) / 1024.0/1024.0 ;
        let cpu_per_process = process.cpu_usage();
        println!("\
        PID: {}, \
        Name: {}, \
        Status: {}, \
        Memory consumed:{:.0} MB,\
        CPU USAGE: {} %", pid, name,status,memory, cpu_per_process);
    }
}


pub fn killprocess(){
    let mut sys = System::new_all();
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();
    let number = input_text.trim().parse::<i32>().unwrap();
    let pid = Pid::from_u32(number as u32);
    sys.refresh_all();

    if let Some(process) = sys.process(pid) {
        println!("Trying to kill {}...", pid);
        match process.kill(){
            true =>{
                sys.refresh_all();
                if sys.process(pid).is_some() {
                    println!("Process is protected or unable to kill...{}", pid);
                }else {
                    println!("Process Killed!");
                }
            },
            false =>{
                println!("Process Couldn´t be killed, pid: {}", pid);
            }
        }

    }else {
        println!("Process not found!");
    }




}