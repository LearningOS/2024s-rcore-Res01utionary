//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;

/// necessary infos of task
#[derive(Copy,Clone)]
pub struct TaskNecInfo {
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// the time when task first run
    pub initial_run_time : usize,
}

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The necessary infos of task, when it first run, task_nec_info get a TaskNecInfo struct value
    pub task_nec_info: Option<TaskNecInfo>,
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
