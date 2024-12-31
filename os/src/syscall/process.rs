//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, translate_user_addr, TaskStatus, get_current_task_info
    }, timer::get_time_us, mm::VirtAddr
};
use crate::mm::MapPermission;
use crate::task::{add_memery_map_to_pagetable, del_memery_map_to_pagetable};
use crate::config::PAGE_SIZE;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let time = get_time_us();
    let sec_vaddr = VirtAddr::from(_ts as usize);
    let usec_vaddr = VirtAddr::from((_ts as usize) + core::mem::size_of::<usize>());
    let sec_paddr = translate_user_addr(sec_vaddr).unwrap();
    let usec_paddr = translate_user_addr(usec_vaddr).unwrap();
    unsafe {
        *(sec_paddr as *mut usize) = time / 1000000;
        *(usec_paddr as *mut usize) = time % 1000000;
    }
    return 0;
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");

    let ti_size = core::mem::size_of::<TaskInfo>();
    unsafe {
        copy_to_user(_ti as *mut u8, ti_size);
    }

    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if (_start % PAGE_SIZE) != 0 {
        return -1;
    }

    if (_port & !0x7) != 0 {
        return -1;
    }
    if (_port & 0x7) == 0 {
        return -1;
    }

    let start = _start;
    let mut permission = MapPermission::U;
    if (_port & 0x1) != 0 { permission |= MapPermission::R; }
    if (_port & 0x2) != 0 { permission |= MapPermission::W; }
    if (_port & 0x4) != 0 { permission |= MapPermission::X; }

    let map_perm = permission;
    if add_memery_map_to_pagetable(start, start + _len, map_perm) {
        return 0;
    }
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    if (_start % PAGE_SIZE) != 0 {
        return -1;
    }
    if del_memery_map_to_pagetable(_start, _start + _len) {
        return 0;
    }
    -1
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}


unsafe fn copy_to_user(dst: *mut u8, size: usize) {
    let mut current_ptr = dst;
    let mut remaining_size = size;
    
    while remaining_size > 0 {
        let vaddr = VirtAddr::from(current_ptr as usize);
        let paddr = translate_user_addr(vaddr).unwrap();
        
        let page_remaining = PAGE_SIZE - vaddr.page_offset();
        let copy_size = remaining_size.min(page_remaining);
        
        // get current task info
        let task_info = get_current_task_info();
        
        // copy data
        core::ptr::copy_nonoverlapping(
            &task_info as *const _ as *const u8,
            paddr as *mut u8,
            copy_size
        );
        
        // update pointer and remaining size
        current_ptr = current_ptr.add(copy_size);
        remaining_size -= copy_size;
    }
}
