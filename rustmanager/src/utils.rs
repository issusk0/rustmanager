use std::*;
use std::time::Duration;
use sysinfo::{Pid, Process, System};


pub fn info() {
    let mut sys = System::new_all();
    loop {



        sys.refresh_all();
        let cpu_usage = sys.global_cpu_usage();
        let core_cpus = sys.cpus().len();
        let total_threads = std::thread::available_parallelism().unwrap();
        let total_memory = sys.total_memory();
        let free_memory = sys.available_memory();
        println!("Total RAM Memory: {} MB", total_memory / 1024 / 1024);
        println!("Available RAM memory: {} MB", free_memory / 1024 / 1024);
        println!("Total CPU usage: {}% ", cpu_usage);
        println!("Total core cpus: {}", core_cpus);
        println!("Total Threads: {}", total_threads);

        let mut processes_list: Vec<(&Pid, &Process)> = sys.processes().into_iter().collect();
        processes_list.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap());
        for (pid, process) in processes_list.iter() {
            let ram_usage = process.memory() / 1024 / 1024;
            let proccess_name = process.name().to_string_lossy();
            let cpu_usage = process.cpu_usage();
            println!("\
            PID: {}\
            -Process Name: {}-\
            CPU usage: {:.2}% \
            RAM usage-{} MB\
            \
            ", pid,proccess_name, cpu_usage, ram_usage);
        };
        println!();
        println!();
        println!("Scanning processes....");
        thread::sleep(Duration::from_millis(5000));
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