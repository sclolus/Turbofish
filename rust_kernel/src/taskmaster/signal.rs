//! This file contains signal interface

use super::process::CpuState;
use super::scheduler::Pid;
use super::SysResult;

use alloc::collections::vec_deque::VecDeque;
use core::convert::TryFrom;
use core::mem;
use core::mem::{size_of, transmute};
use core::ops::{Index, IndexMut};
use errno::Errno;

extern "C" {
    static _trampoline: u8;
    static _trampoline_len: u32;
}

/// allign on
#[inline(always)]
pub fn align_on(t: usize, on: usize) -> usize {
    debug_assert!(on.is_power_of_two());
    if t & (on - 1) == 0 {
        t
    } else {
        t + (on - (t & (on - 1)))
    }
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
#[repr(u32)]
pub enum Signum {
    SigNull = 0,
    /// Hangup (POSIX).
    Sighup = 1,
    /// Interrupt (ANSI).
    Sigint = 2,
    /// Quit (POSIX).
    Sigquit = 3,
    /// Illegal instruction (ANSI).
    Sigill = 4,
    /// Trace trap (POSIX).
    Sigtrap = 5,
    /// Abort (ANSI).
    Sigabrt = 6,
    /// BUS error (4.2 BSD).
    Sigbus = 7,
    /// Floating-point exception (ANSI).
    Sigfpe = 8,
    /// Kill, unblockable (POSIX).
    Sigkill = 9,
    /// User-defined signal 1 (POSIX).
    Sigusr1 = 10,
    /// Segmentation violation (ANSI).
    Sigsegv = 11,
    /// User-defined signal 2 (POSIX).
    Sigusr2 = 12,
    /// Broken pipe (POSIX).
    Sigpipe = 13,
    /// Alarm clock (POSIX).
    Sigalrm = 14,
    /// Termination (ANSI).
    Sigterm = 15,
    /// Stack fault.
    Sigstkflt = 16,
    /// Child status has changed (POSIX).
    Sigchld = 17,
    /// Continue (POSIX).
    Sigcont = 18,
    /// Stop, unblockable (POSIX).
    Sigstop = 19,
    /// Keyboard stop (POSIX).
    Sigtstp = 20,
    /// Background read from tty (POSIX).
    Sigttin = 21,
    /// Background write to tty (POSIX).
    Sigttou = 22,
    /// Urgent condition on socket (4.2 BSD).
    Sigurg = 23,
    /// CPU limit exceeded (4.2 BSD).
    Sigxcpu = 24,
    /// File size limit exceeded (4.2 BSD).
    Sigxfsz = 25,
    /// Virtual alarm clock (4.2 BSD).
    Sigvtalrm = 26,
    /// Profiling alarm clock (4.2 BSD).
    Sigprof = 27,
    /// Window size change (4.3 BSD, Sun).
    Sigwinch = 28,
    /// I/O now possible (4.2 BSD).
    Sigio = 29,
    /// Power failure restart (System V).
    Sigpwr = 30,
    /// Bad system call.
    Sigsys = 31,
}

pub enum DefaultAction {
    Abort,
    Terminate,
    Ignore,
    Stop,
    Continue,
}

pub fn signal_default_action(signum: Signum) -> DefaultAction {
    use Signum::*;
    match signum {
        Sigstkflt | Sigabrt | Sigbus | Sigfpe | Sigill | Sigquit | Sigsegv | Sigsys | Sigtrap | Sigxcpu | Sigxfsz => {
            DefaultAction::Abort
        }

        Sigalrm | Sighup | Sigint | Sigkill | Sigpipe | Sigusr1 | Sigusr2 | Sigprof | Sigvtalrm | Sigterm => {
            DefaultAction::Terminate
        }

        Sigcont => DefaultAction::Continue,

        Sigio | Sigpwr | Sigwinch | SigNull | Sigchld | Sigurg => DefaultAction::Ignore,

        Sigstop | Sigtstp | Sigttin | Sigttou => DefaultAction::Stop,
    }
}

#[derive(Debug)]
pub struct InvalidSignum;

impl TryFrom<u32> for Signum {
    type Error = InvalidSignum;
    fn try_from(n: u32) -> Result<Self, Self::Error> {
        if n >= 32 {
            return Err(InvalidSignum);
        } else {
            Ok(unsafe { transmute(n) })
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum Sigaction {
    SigDfl,
    SigIgn,
    Handler(extern "C" fn(i32)),
}

#[derive(Copy, Clone, Debug)]
pub struct SignalActions(pub [Sigaction; 32]);

impl IndexMut<Signum> for SignalActions {
    fn index_mut(&mut self, index: Signum) -> &mut Sigaction {
        &mut self.0[index as usize]
    }
}

impl Index<Signum> for SignalActions {
    type Output = Sigaction;
    fn index(&self, index: Signum) -> &Sigaction {
        &self.0[index as usize]
    }
}

#[derive(Debug)]
pub struct SignalInterface {
    pub signal_actions: SignalActions,
    pub signal_queue: VecDeque<Signum>,
    pub signaled: bool,
}

impl SignalInterface {
    pub fn new() -> Self {
        Self { signal_actions: SignalActions([Sigaction::SigDfl; 32]), signal_queue: VecDeque::new(), signaled: false }
    }

    pub fn is_signaled(&self) -> bool {
        self.signaled
    }

    pub fn set_signaled(&mut self, b: bool) {
        self.signaled = b;
    }

    pub fn signal(&mut self, signum: u32, handler: extern "C" fn(i32)) -> SysResult<u32> {
        let signum = Signum::try_from(signum).map_err(|_| Errno::Einval)?;
        let former = mem::replace(&mut self.signal_actions[signum], Sigaction::Handler(handler));
        match former {
            Sigaction::Handler(h) => Ok(h as u32),
            _ => Ok(0),
        }
    }

    pub fn kill(&mut self, signum: u32) -> SysResult<u32> {
        let signum = Signum::try_from(signum).map_err(|_| Errno::Einval)?;
        self.signal_queue.try_reserve(1)?;
        self.signal_queue.push_back(signum);
        Ok(0)
    }

    pub fn has_pending_signals(&self) -> bool {
        !self.signal_queue.is_empty()
    }
    pub fn exec_signal_handler(&mut self, signum: Signum, kernel_esp: u32, f: extern "C" fn(i32)) {
        /// helper to push on the stack
        /// imitate push instruction by incrementing esp before push t
        fn push_esp<T: Copy>(esp: &mut u32, t: T) {
            if size_of::<T>() % 4 != 0 {
                panic!("size not multiple of 4");
            }
            *esp -= size_of::<T>() as u32;
            unsafe {
                (*esp as *mut T).write(t);
            }
        }

        /// helper to push on the stack
        /// same as push_esp but buf a `buf` of size `size`
        fn push_buff_esp(esp: &mut u32, buf: *mut u8, size: usize) {
            // align size
            let size = align_on(size, 4);
            *esp -= size as u32;
            unsafe {
                (*esp as *mut u8).copy_from(buf, size);
            }
        }
        //debug_assert!(kernel_esp > self.unwrap_running().kernel_stack.as_ptr() as u32);
        unsafe {
            let cpu_state: *mut CpuState = kernel_esp as *mut CpuState;

            // dbg_hex!(*cpu_state);
            // eprintln!("{:?}", *cpu_state);
            // TODO: check if interruptable
            let mut user_esp = if !(*cpu_state).run_in_ring3() {
                // if in a syscall and running do not perfom signal handling
                // if self.is_running() {
                //TODO: handle that cases, panic for the moment
                // panic!("is running");
                //self.signal_queue.push_front(signum);
                //return;
                //} else {
                //panic!("is not ring3");
                // get the cpu state at the base of the kernel stack
                //(*cpu_state).esp = kernel_esp;
                //let syscall_cpu_state: CpuState = *((self.unwrap_running().kernel_stack_base()
                //    - size_of::<CpuState>() as u32)
                //    as *const CpuState);
                // dbg_hex!(syscall_cpu_state);
                //syscall_cpu_state.esp
                //}
                0
            } else {
                // eprintln!("is in ring3");
                (*cpu_state).esp
            };
            // dbg_hex!(user_esp);
            // dbg!(cpu_state);
            // push the current cpu_state on the user stack
            push_esp(&mut user_esp, *cpu_state);
            // push the trampoline code on the user stack
            push_buff_esp(&mut user_esp, symbol_addr!(_trampoline) as *mut u8, _trampoline_len as usize);
            // push the address of start of trampoline code stack on the user stack
            let esp_trampoline = user_esp;
            push_esp(&mut user_esp, signum as u32);
            push_esp(&mut user_esp, esp_trampoline);

            // set a fresh cpu state to execute the handler
            let mut new_cpu_state = CpuState::new(user_esp, f as u32);
            new_cpu_state.eip = f as u32;

            (*cpu_state) = new_cpu_state;
            (*cpu_state).eip = f as u32;
            // dbg_hex!(*cpu_state);
        }
        self.set_signaled(true);
    }

    /// sigreturn syscall
    pub fn sigreturn(&mut self, cpu_state: *mut CpuState) -> SysResult<u32> {
        /// helper to push on the stack
        /// imitate pop instruction return the T present at esp
        fn pop_esp<T: Copy>(esp: &mut u32) -> T {
            if size_of::<T>() % 4 != 0 {
                panic!("size not multiple of 4");
            }
            unsafe {
                let t = *(*esp as *mut T);
                *esp += size_of::<T>() as u32;
                t
            }
        }

        if !self.is_signaled() {
            panic!("can't call sigreturn when not interrupted");
        }
        unsafe {
            eprintln!("sigreturn");
            // dbg_hex!(*cpu_state);
            // skip the trampoline code
            (*cpu_state).esp += align_on(_trampoline_len as usize, 4) as u32;
            // get back the old cpu state and set it as the current cpu_state
            let _signum: u32 = pop_esp(&mut (*cpu_state).esp);
            let old_cpu_state: CpuState = pop_esp(&mut (*cpu_state).esp);
            // dbg_hex!(old_cpu_state);
            *cpu_state = old_cpu_state;

            self.set_signaled(false);
            // return current eax to keep it's value at the syscall return
            Ok((*cpu_state).registers.eax)
        }
    }

    /// check if there is pending sigals, and tricks the stack to execute it on return
    pub fn check_pending_signals(&mut self, kernel_esp: u32, pid: Pid) {
        // eprintln!("check pending signals");
        // let task = self.get_process_mut(pid).expect("no task with that pid");

        if !self.is_signaled() {
            if let Some(signum) = self.signal_queue.pop_front() {
                match self.signal_actions[signum] {
                    Sigaction::Handler(f) => self.exec_signal_handler(signum, kernel_esp, f),
                    Sigaction::SigDfl => {
                        use DefaultAction::*;
                        match signal_default_action(signum) {
                            Abort => {
                                //TODO: Exit the process  status
                                //self.exit(status: i32)
                            }
                            Terminate => {
                                //TODO: Exit the process  status
                                //self.exit(status: i32)
                            }
                            Ignore => {
                                return self.check_pending_signals(pid, kernel_esp);
                            }
                            Continue => unimplemented!(),
                            Stop => unimplemented!(),
                        }
                    }
                    Sigaction::SigIgn => {
                        return self.check_pending_signals(pid, kernel_esp);
                    }
                }
            }
        }
    }
}
