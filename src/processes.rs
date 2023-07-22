extern crate winapi;

use std::ffi::CStr;
use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::um::handleapi::CloseHandle;
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
};


fn process_snapshot() -> Option<*mut winapi::ctypes::c_void> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
            eprintln!("Failed to create snapshot of current processes.");
            None
        } else {
            Some(snapshot)
        }
    }
}

fn first_process_entry(snapshot: *mut winapi::ctypes::c_void) -> Option<PROCESSENTRY32> {
    let mut pe32: PROCESSENTRY32 = unsafe { std::mem::zeroed() };
    pe32.dwSize = std::mem::size_of::<PROCESSENTRY32>() as DWORD;
    let success = unsafe { Process32First(snapshot, &mut pe32) };

    if success == FALSE {
        eprintln!("Failed to gather information about the first process.");
        unsafe { CloseHandle(snapshot) };
        None
    } else {
        Some(pe32)
    }
}

fn iterate_processes<F: FnMut(PROCESSENTRY32) -> bool>(mut callback: F) {
    if let Some(snapshot) = process_snapshot() {
        if let Some(mut pe32) = first_process_entry(snapshot) {
            loop {
                if !callback(pe32) {
                    break;
                }

                if unsafe { Process32Next(snapshot, &mut pe32) } == FALSE {
                    break;
                }
            }

            unsafe { CloseHandle(snapshot) };
        }
    }
}

pub fn enumerate_processes() {
    iterate_processes(|pe32| {
        let process_name = unsafe {
            CStr::from_ptr(pe32.szExeFile.as_ptr())
                .to_string_lossy()
                .into_owned()
        };
        println!(
            "Process ID: {}, Process Name: {}",
            pe32.th32ProcessID, process_name
        );
        true
    });
}


pub fn find_process_id_by_name(name: &str) -> Option<DWORD> {
    let name = name.to_lowercase();
    let mut pid = None;

    iterate_processes(|pe32| {
        let process_name = unsafe {
            CStr::from_ptr(pe32.szExeFile.as_ptr())
                .to_string_lossy()
                .into_owned()
        };

        if process_name.to_lowercase() == name {
            println!(
                "Found process: {}, Process ID: {}",
                process_name, pe32.th32ProcessID
            );
            pid = Some(pe32.th32ProcessID);
            false // stop iterating
        } else {
            true // continue iterating
        }
    });

    if pid.is_none() {
        eprintln!("No process found with name: {}", name);
    }

    pid
}