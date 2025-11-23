use std::*;
use std::time::Duration;
use sysinfo::{Pid, Process, System, Disks};


pub fn task_info() {
     let mut sys = System::new_all();
     let mut disks = Disks::new_with_refreshed_list();
    loop {

        cpu_ram_info(&mut sys);
        processes_info(&mut sys);
        disk_info(&mut disks);




        println!();
        println!();
        killprocess(&mut sys);
     
        println!("Scanning processes....");
    }
    

}

fn cpu_ram_info(sys: &mut System){
        sys.refresh_all();
        thread::sleep(Duration::from_secs(1));
        sys.refresh_all();
        let cpu_usage = sys.global_cpu_usage();
        let core_cpus = sysinfo::System::physical_core_count().unwrap();
        let total_threads = sys.cpus().len();
        let total_memory = sys.total_memory();
        let free_memory = sys.available_memory();

        
        println!("Total RAM Memory: {} MB", total_memory / 1024 / 1024);
        println!("Available RAM memory: {} MB", free_memory / 1024 / 1024);
        println!("Total CPU usage: {}% ", cpu_usage);
        println!("Total logical cores: {}", core_cpus);
        println!("Total Threads: {}", total_threads);
        thread::sleep(Duration::from_secs(5));
}
fn processes_info(sys: &mut System){
        sys.refresh_all();
        println!("Filtered by % usage of CPU");
        let mut processes_list: Vec<(&Pid, &Process)> = sys.processes().into_iter().collect();
        processes_list.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap());
        for (pid, process) in processes_list.iter() {
            let ram_usage = process.memory() / 1024 / 1024;
            let proccess_name = process.name().to_string_lossy();
            let cpu_usage = process.cpu_usage();
            println!("\
            PID: {} \
            Process Name: {} \
            CPU usage: {:.2}% \
            RAM usage: {} MB \
            \
            ", pid,proccess_name, cpu_usage, ram_usage);
        };

}
fn disk_info(disks: &mut Disks){
       for disk in disks.list(){
            let name = disk.name().to_string_lossy();
            let total_space = disk.total_space();
            let available_space = disk.available_space();
            let used_space = total_space - available_space;
            let used_percentage = (used_space as f64 / total_space as f64) * 100.0;
            
            println!("Disk name: {} \
                      Total storage used {:.1}%", name,used_percentage)
        }

}
fn killprocess(sys: &mut System) {
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();
    let number = input_text.trim().parse::<i32>().unwrap();
    let pid = Pid::from_u32(number as u32);
    sys.refresh_all();


    while true{
        if let Some(process) = sys.process(pid) {
            println!("Trying to kill {}...", pid);
            match process.kill(){
                false =>{
                    sys.refresh_all();
                    if sys.process(pid).is_some() {
                        println!("Unable to kill the function {} due to lack of privileges", pid);
                        break
                    }else{
                        break;
                    }
                },
                true =>{
                    println!("Process has been killed successufylly! :D");
                    break;
                }
            }

        }else {
            println!("Invaild PID, proccess does not exists!");
            break;
        }
    }





}