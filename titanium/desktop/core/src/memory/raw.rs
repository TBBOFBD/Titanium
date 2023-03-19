#![allow(unused_imports)]

use winapi::{
    shared::{minwindef::{DWORD, LPCVOID, LPVOID}, ntdef::{ULONG, FALSE}},
    um::{
        processthreadsapi::OpenProcess,
        memoryapi::{ReadProcessMemory, WriteProcessMemory},
        tlhelp32::{
            Process32FirstW, Process32NextW, Module32FirstW, Module32NextW, TH32CS_SNAPMODULE, TH32CS_SNAPPROCESS, MODULEENTRY32W,
            PROCESSENTRY32W, CreateToolhelp32Snapshot
        },
        handleapi::CloseHandle,
        winnt::{HANDLE, PROCESS_ALL_ACCESS},
    },
    ctypes::c_void,
};
use std::{mem::zeroed, ptr::{null_mut, null}, ffi::OsStr, os::windows::ffi::OsStrExt};

use super::MemResult;

pub(crate) struct RawMemoryManager {
    process_id: DWORD,
    process_handle: HANDLE,
}

// GetProcessEntryByName: Same as constructor in the C++ code which finds the process id and opens a handle.
impl RawMemoryManager {
    pub(crate) fn new(name: String) -> MemResult<Self> {
        unsafe {
            let mut entry: PROCESSENTRY32W = zeroed();
            entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

            // Create tool help snapshot to enumerate processes.
            let snapshot = CreateToolhelp32Snapshot(2, 0);

            // Check if snapshot is valid
            // if Process32FirstW(snapshot, &mut entry) == 0 {
            //     CloseHandle(snapshot);
            //     return Err(error_maker("Failed finding any available process 1"));
            // }
            
            while Process32NextW(snapshot, &mut entry) != 0 {
                let exe_file = String::from_utf16_lossy(&entry.szExeFile.to_vec()); // Max filename length
                let exe_file = exe_file.replace("\0", "");
                if name.to_lowercase() == exe_file.to_lowercase() {
                    println!("Found process: {}", exe_file);
                    let process_id = entry.th32ProcessID;
                    let process_handle = OpenProcess(PROCESS_ALL_ACCESS, 0, process_id);
    
                    return Ok(
                        RawMemoryManager { process_id, process_handle }
                    );
                }

                // let exe_file = String::from_utf16_lossy(&entry.szExeFile.to_vec()); // Max filename length
                // let exe_file = exe_file.replace("\0", "");
                // if exe_file.to_lowercase() == name {
                //     let process_id = entry.th32ProcessID;
                //     let process_handle = OpenProcess(PROCESS_ALL_ACCESS, 0, process_id);
                //     println!("process_handle ID: {:?}", &process_handle);
                //     if process_handle == null_mut() {
                //         CloseHandle(snapshot);
                //         return Err(error_maker("Failed opening process handle 2"));
                //     }

                //     CloseHandle(snapshot);

                //     return Ok(
                //         RawMemoryManager { process_id, process_handle }
                //     );
                // }
            }

            CloseHandle(snapshot);
            return Err(error_maker("Failed finding target process 3"));
        }
    }

    // GetModuleAddress: Returns the base address of a module by name
    pub(crate) fn get_module_address(&self, module_name: String) -> MemResult<ULONG> {
        unsafe {
            let mut entry: MODULEENTRY32W = zeroed();
            entry.dwSize = std::mem::size_of::<MODULEENTRY32W>() as u32;

            let snapshot = CreateToolhelp32Snapshot(
                TH32CS_SNAPMODULE,
                self.process_id
            );
            
            if Module32FirstW(snapshot, &mut entry) == 0 {
                CloseHandle(snapshot);
                return Err(error_maker("Failed finding module."));
            }
            
            while Module32NextW(snapshot, &mut entry) != 0 {
                let name = String::from_utf16_lossy(&entry.szModule[..256]);

                if name.to_lowercase() == module_name {
                    CloseHandle(snapshot);
                    return Ok(entry.modBaseAddr as ULONG);
                }
            }

            CloseHandle(snapshot);
            return Err(error_maker("Failed finding module."));
        }
    }

    pub(crate) fn read_memory<T>(&self, address: ULONG) -> MemResult<T> {
        unsafe {
            let mut data = std::mem::zeroed::<T>();
            let mut read: usize = 0;
            let result = ReadProcessMemory(
                self.process_handle,
                address as LPCVOID,
                &mut data as *mut _ as LPVOID,
                std::mem::size_of::<T>(),
                &mut read
            );
            if result == 0 {
                return Err(error_maker("Failed reading memory."));
            }

            Ok(data)
        }
    }

    pub(crate) fn write_memory<T>(&self, address: ULONG, value: T) -> MemResult<()> {
        unsafe {
            let mut written: usize = 0;
            
            let result = WriteProcessMemory(
                self.process_handle, 
                address as *mut c_void,
                &value as *const _ as *const c_void,
                std::mem::size_of::<T>(),
                &mut written,
            );

            if result == 0 {
                return Err(error_maker("Failed writing memory."));
            }

            Ok(())
        }
    }

    pub(crate) fn read_raw<T>(address: u64) -> MemResult<T> {
        unsafe {
            let p = address as *const T;
            let res: T = std::ptr::read(p);
            return Ok(res);
        }
    }
}

fn error_maker(msg: &str) -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::Other,
        format!(
            "[MEMORY] {}",
            msg
        )
    )
}