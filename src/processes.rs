extern crate winapi;

use std::ffi::CStr;
use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::um::handleapi::CloseHandle;
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
};

fn process_snapshot() -> Result<*mut winapi::ctypes::c_void, &'static str> {
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        Err("Failed to create snapshot of current processes.")
    } else {
        Ok(snapshot)
    }
}

fn first_process_entry(
    snapshot: *mut winapi::ctypes::c_void,
) -> Result<PROCESSENTRY32, &'static str> {
    let mut pe32: PROCESSENTRY32 = unsafe { std::mem::zeroed() };
    pe32.dwSize = std::mem::size_of::<PROCESSENTRY32>() as DWORD;
    let success = unsafe { Process32First(snapshot, &mut pe32) };

    if success == FALSE {
        unsafe { CloseHandle(snapshot) };
        Err("Failed to gather information about the first process.")
    } else {
        Ok(pe32)
    }
}

fn iterate_processes<F: FnMut(PROCESSENTRY32) -> bool>(
    mut callback: F,
) -> Result<(), &'static str> {
    let snapshot = process_snapshot()?;
    let mut pe32 = first_process_entry(snapshot)?;

    loop {
        if !callback(pe32) || unsafe { Process32Next(snapshot, &mut pe32) } == FALSE {
            break;
        }
    }

    unsafe { CloseHandle(snapshot) };
    Ok(())
}

pub fn enumerate_processes() {
    let _ = iterate_processes(|pe32| {
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

    let _ = iterate_processes(|pe32| {
        let process_name = unsafe {
            CStr::from_ptr(pe32.szExeFile.as_ptr())
                .to_string_lossy()
                .into_owned()
        };

        let is_target_process_found = process_name.to_lowercase() == name;

        if is_target_process_found {
            println!(
                "Found process: {}, Process ID: {}",
                process_name, pe32.th32ProcessID
            );
            pid = Some(pe32.th32ProcessID);
        }

        !is_target_process_found
    });

    if pid.is_none() {
        eprintln!("No process found with name: {}", name);
    }

    pid
}
