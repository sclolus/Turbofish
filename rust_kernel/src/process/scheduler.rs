use crate::process::Process;
use crate::registers::Eflags;
use crate::spinlock::Spinlock;
use crate::system::BaseRegisters;
use alloc::vec;
use alloc::vec::Vec;
use lazy_static::lazy_static;

extern "C" {
    fn _process_a();
    fn _process_b();
    fn _switch_process(eflags: u32, segment: u32, eip: u32, esp: u32, registers: BaseRegisters) -> !;
}

#[no_mangle]
unsafe extern "C" fn timer_interrupt_handler(
    old_eip: u32,
    old_segment: u32,
    old_eflags: u32,
    old_esp: u32,
    registers: BaseRegisters,
) -> ! {
    SCHEDULER.lock().interrupt(old_eip, old_segment, old_eflags, old_esp, registers);
}

pub struct Scheduler {
    process: Vec<Process>,
    curr_process_index: usize,
    no_process: bool,
}

impl Scheduler {
    unsafe fn new() -> Self {
        Self {
            process: vec![
                //TODO: change the get_eflags for some default flags
                Process::new(_process_a, Eflags::get_eflags().set_interrupt_flag(true)),
                Process::new(_process_b, Eflags::get_eflags().set_interrupt_flag(true)),
            ],
            curr_process_index: 0,
            no_process: true,
        }
    }
    unsafe fn interrupt(
        &mut self,
        old_eip: u32,
        old_segment: u32,
        old_eflags: u32,
        old_esp: u32,
        registers: BaseRegisters,
    ) -> ! {
        let eflags = crate::registers::Eflags::new(old_eflags);
        // eprintln!("sched");
        // eprintln!("segment: {:X}, eflags: {}", old_segment, eflags);
        // eprintln!("eip:{:X?} esp:{:X?} reg:{:X?}", old_eip, old_esp, registers);
        if self.no_process {
            self.no_process = false;
            return self.switch_process(0);
        }
        let next_process = (self.curr_process_index + 1) % self.process.len();
        let curr_process: &mut Process = &mut self.process[self.curr_process_index];
        curr_process.eip = old_eip;
        curr_process.esp = old_esp;
        curr_process.registers = registers;
        curr_process.eflags = eflags;
        curr_process.segment = old_segment;
        self.switch_process(next_process)
    }

    unsafe fn switch_process(&mut self, next_process_index: usize) -> ! {
        let next_process: &Process = &self.process[next_process_index];
        // eprintln!(
        //     "switch to eip:{:X?} esp:{:X?} reg:{:X?}\n eflags: {}",
        //     next_process.eip, next_process.esp, next_process.registers, next_process.eflags
        // );
        next_process.virtual_allocator.context_switch();
        self.curr_process_index = next_process_index;
        SCHEDULER.force_unlock();
        _switch_process(
            next_process.eflags.inner(),
            next_process.segment,
            next_process.eip,
            next_process.esp,
            next_process.registers,
        );
    }
}

lazy_static! {
    pub static ref SCHEDULER: Spinlock<Scheduler> = Spinlock::new(unsafe { Scheduler::new() });
}

#[no_mangle]
extern "C" fn process_a() {
    unsafe {
        for i in 0..1000000 {
            user_eprintln!("process A {}", i);
            asm!("hlt"::::"volatile");
        }
    }
}

#[no_mangle]
extern "C" fn process_b() {
    unsafe {
        for i in 0..1000000 {
            user_eprintln!("process B {}", i);
            asm!("hlt"::::"volatile");
        }
    }
}

#[no_mangle]
extern "C" fn debug_process(eip: u32) {
    eprintln!("jump to eip |{:X}|\n", eip);
    crate::tests::helpers::exit_qemu(0);
    loop {}
}
