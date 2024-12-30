//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The time of the task
    pub time: usize,
    /// The syscall times of the task
    pub syscall_times: [usize; MAX_SYSCALL_NUM],
}

impl TaskControlBlock {
    /// Get the current status of the task
    pub fn get_task_status(&self) -> TaskStatus {
        self.task_status
    }
    
    /// Get the running time of the task
    pub fn get_time(&self) -> usize {
        self.time
    }
    
    /// Get the syscall invocation statistics of the task
    pub fn get_syscall_times(&self) -> [u32; MAX_SYSCALL_NUM] {
        self.syscall_times.map(|x| x as u32)
    }
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
