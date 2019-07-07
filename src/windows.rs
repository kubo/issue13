use std::mem::size_of;
use winapi::shared::minwindef::DWORD;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::processthreadsapi::{GetCurrentProcess, GetProcessHandleCount};
use winapi::um::psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

// https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa965225(v=vs.85)#process-memory-performance-information
pub fn print_memory_usage_header() {
    println!("Handle Count, Page File Bytes, Virtual Bytes, Working Set");
}

pub fn print_memory_usage() {
    let mut handle_count = 0;
    let mut pmc = PROCESS_MEMORY_COUNTERS::default();
    let size = size_of::<PROCESS_MEMORY_COUNTERS>() as DWORD;
    let mut ms = MEMORYSTATUSEX::default();
    pmc.cb = size;
    ms.dwLength = size_of::<MEMORYSTATUSEX>() as DWORD;

    unsafe {
        let handle = GetCurrentProcess();
        if GetProcessHandleCount(handle, &mut handle_count) == 0 {
            println!("GetProcessHandleCount error: {}", GetLastError());
        }
        if GetProcessMemoryInfo(handle, &mut pmc, size) == 0 {
            println!("GetProcessMemoryInfo error: {}", GetLastError());
        }
        if GlobalMemoryStatusEx(&mut ms) == 0 {
            println!("GlobalMemoryStatusEx error: {}", GetLastError());
        }
    }
    println!(
        "{}, {}, {}, {}",
        handle_count,
        pmc.PagefileUsage,
        ms.ullTotalVirtual - ms.ullAvailVirtual,
        pmc.WorkingSetSize
    );
}
