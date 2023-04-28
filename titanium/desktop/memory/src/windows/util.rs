//! Utility functions for Windows

#![cfg(target_os = "windows")]
#![allow(unused_imports)]

use std::ffi::CString;
use std::path::Path;
use sysinfo::{ProcessExt, SystemExt};
use winapi::shared::minwindef::{DWORD, FALSE, LPCVOID};
use winapi::shared::ntdef::NULL;
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryA};
use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
use winapi::um::processthreadsapi::{CreateRemoteThread, OpenProcess};
use winapi::um::winnt::{MEM_COMMIT, PAGE_EXECUTE_READWRITE, PROCESS_ALL_ACCESS};

/// The error type for [`inject_dll_into_process`]
#[derive(Debug, thiserror::Error)]
pub enum InjectDllIntoProcessError {
    /// Failed to canonicalize dll path
    #[error("Failed to canonicalize dll path")]
    DllPathCanonicalizationFailed(std::io::Error),
    /// Failed to convert dll path to string
    #[error("Failed to stringify dll path")]
    DllPathToStrFailed,
    /// Failed to open process
    #[error("Open process failed")]
    OpenProcessFailed,
    /// Failed to allocate memory in process
    #[error("Dll path buffer allocation failed")]
    DllPathBufferAllocationFailed,
    /// Failed to write dll path to process memory
    #[error("Writing process memory failed (code: {err_code:?})")]
    WritingProcessMemoryFailed { err_code: i32 },
    /// Failed to get kernel32 handle
    #[error("Kernel32 not found")]
    Kernel32NotFound,
    /// Failed to get LoadLibraryA address
    #[error("LoadLibraryA not found")]
    LoadLibraryANotFound,
    /// Failed to get CreateRemoteThread address
    #[error("Failed to create remote thread")]
    FailedToCreateRemoteThread,
}

/// Injects a DLL into a process
/// `process_id` The process id
/// `dll_path` The path of the dll that should be injected
pub unsafe fn inject_dll_into_process(
    process_id: crate::Pid,
    dll_path: &Path,
) -> Result<(), InjectDllIntoProcessError> {
    let canonicalized_dll_path = dll_path
        .canonicalize()
        .map_err(|e| InjectDllIntoProcessError::DllPathCanonicalizationFailed(e))?;
    let dll_path_str = canonicalized_dll_path
        .to_str()
        .ok_or_else(|| InjectDllIntoProcessError::DllPathToStrFailed)?;
    let dll_path_cstr = CString::new(dll_path_str).expect("Failed to create CString");
    let dll_path_cstr_len = dll_path_cstr.as_bytes().len() + 1;

    let h_proc = OpenProcess(PROCESS_ALL_ACCESS, FALSE, process_id);
    if h_proc.is_null() {
        return Err(InjectDllIntoProcessError::OpenProcessFailed);
    }
    let dll_path_buffer = VirtualAllocEx(
        h_proc,
        NULL,
        dll_path_cstr_len,
        MEM_COMMIT,
        PAGE_EXECUTE_READWRITE,
    );
    if dll_path_buffer.is_null() {
        return Err(InjectDllIntoProcessError::DllPathBufferAllocationFailed);
    }

    let write_process_memory_err_code = WriteProcessMemory(
        h_proc,
        dll_path_buffer,
        std::mem::transmute::<_, LPCVOID>(dll_path_cstr.as_ptr()),
        dll_path_cstr_len,
        std::ptr::null_mut(),
    );
    if write_process_memory_err_code == 0 {
        return Err(InjectDllIntoProcessError::WritingProcessMemoryFailed {
            err_code: write_process_memory_err_code,
        });
    }
    let kernel32_cstr = CString::new("kernel32").expect("Failed to create CString");
    let kernel32_hinstance = LoadLibraryA(kernel32_cstr.as_ptr());
    if kernel32_hinstance.is_null() {
        return Err(InjectDllIntoProcessError::Kernel32NotFound);
    }

    let load_library_a_cstr = CString::new("LoadLibraryA").expect("Failed to create CString");
    let load_library_addr = GetProcAddress(kernel32_hinstance, load_library_a_cstr.as_ptr());

    if load_library_addr.is_null() {
        return Err(InjectDllIntoProcessError::LoadLibraryANotFound);
    }

    let mut thread_id: DWORD = std::mem::zeroed();
    let thread_result = CreateRemoteThread(
        h_proc,
        std::mem::transmute(NULL),
        0,
        std::mem::transmute(load_library_addr),
        dll_path_buffer,
        0,
        &mut thread_id as *mut DWORD,
    );
    if thread_result.is_null() {
        return Err(InjectDllIntoProcessError::FailedToCreateRemoteThread);
    }

    Ok(())
}