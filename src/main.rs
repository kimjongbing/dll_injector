extern crate winapi;

use std::env;
use winapi::shared::minwindef::DWORD;

mod injection;
mod processes;

fn print_usage() {
    println!("Usage:");
    println!("1. List all processes: dll_injector.exe <list>");
    println!("2. Get PID of a process by name: dll_injector.exe pid <process name>");
    println!("3. Inject DLL into a process: dll_injector.exe <PID> <DLL path>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 if args[1] == "list" => processes::enumerate_processes(),
        3 if args[1] == "pid" => {
            if let Some(pid) = processes::find_process_id_by_name(&args[2]) {
                println!("PID of {}: {}", args[2], pid);
            } else {
                println!("Process not found: {}", args[2]);
            }
        }
        3 if args[1].parse::<DWORD>().is_ok() => {
            let pid = args[1].parse::<DWORD>().unwrap();
            match injection::inject_dll(pid, &args[2]) {
                Ok(_) => println!("Successfully injected DLL into process: {}", pid),
                Err(e) => eprintln!("Failed to inject DLL into process: {}. Error: {}", pid, e),
            }
        }
        _ => print_usage(),
    }
}
