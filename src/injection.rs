extern crate winapi;

use std::ffi::CString;
use std::ptr::null_mut;
use winapi::shared::minwindef::DWORD;
use winapi::um::handleapi::CloseHandle;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::memoryapi::VirtualAllocEx;
use winapi::um::processthreadsapi::{CreateRemoteThread, OpenProcess};
use winapi::um::winnt::{MEM_COMMIT, PAGE_READWRITE, PROCESS_ALL_ACCESS};

pub fn inject_dll(pid: DWORD, dll_path: &str) -> Result<(), String> {
    let dll_path_cstring = CString::new(dll_path.to_string()).expect("CString::new failed");

    unsafe {
        let process = OpenProcess(PROCESS_ALL_ACCESS, 0, pid);
        if process.is_null() {
            return Err("Failed to open the target process.".to_string());
        }

        let addr = VirtualAllocEx(
            process,
            null_mut(),
            dll_path_cstring.to_bytes_with_nul().len(),
            MEM_COMMIT,
            PAGE_READWRITE,
        );
        if addr.is_null() {
            return Err("Failed to allocate memory in the target process.".to_string());
        }

        if winapi::um::memoryapi::WriteProcessMemory(
            process,
            addr,
            dll_path_cstring.as_ptr() as *const _,
            dll_path_cstring.to_bytes_with_nul().len(),
            null_mut(),
        ) == 0
        {
            return Err("Failed to write into the target process memory.".to_string());
        }

        let kernel32 = CString::new("kernel32.dll").expect("CString::new failed");
        let loadlibrarya = CString::new("LoadLibraryA").expect("CString::new failed");

        let h_kernel32 = GetModuleHandleA(kernel32.as_ptr());
        if h_kernel32.is_null() {
            return Err("Failed to get the handle of kernel32.dll.".to_string());
        }

        let h_loadlibrarya =
            winapi::um::libloaderapi::GetProcAddress(h_kernel32, loadlibrarya.as_ptr());
        if h_loadlibrarya.is_null() {
            return Err("Failed to get the address of LoadLibraryA.".to_string());
        }

        if CreateRemoteThread(
            process,
            null_mut(),
            0,
            Some(std::mem::transmute(h_loadlibrarya)),
            addr as *mut _,
            0,
            null_mut(),
        )
        .is_null()
        {
            return Err("Failed to create a remote thread in the target process.".to_string());
        }

        CloseHandle(process);
    }

    Ok(())
}
