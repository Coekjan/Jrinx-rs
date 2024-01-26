#![no_std]
#![feature(asm_const)]
#![feature(offset_of)]

#[macro_use]
extern crate log;

pub mod arch;
pub mod breakpoint;
pub mod soft_int;
pub mod timer_int;

use core::fmt::Debug;

use jrinx_addr::VirtAddr;
use jrinx_paging::PagePerm;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrapReason {
    ExternalInterrupt,
    SoftwareInterrupt,
    TimerInterrupt,
    SystemCall,
    Breakpoint { addr: VirtAddr },
    PageFault { addr: VirtAddr, perm: PagePerm },
    Unknown { code: usize },
}

pub trait GenericContext: Debug + Clone + Copy {
    fn trap_reason(&self) -> TrapReason;

    fn syscall_num(&self) -> usize;

    fn syscall_args(&self) -> [usize; 7];

    fn syscall_ret(&mut self, ret: usize);

    fn user_setup(&mut self, entry_point: usize, stack_top: usize);

    fn enable_int(&mut self);

    fn disable_int(&mut self);

    fn pc_advance(&mut self);

    fn run(&mut self);
}

pub fn init() {
    arch::init();
}
