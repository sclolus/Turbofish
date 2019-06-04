use super::process::{CpuState, UserProcess};
use super::Pid;
use super::SysResult;

use errno::Errno;

use alloc::boxed::Box;
use alloc::collections::vec_deque::VecDeque;
use alloc::vec::Vec;

use core::convert::TryFrom;
use core::mem;
use core::mem::{size_of, transmute};
use core::ops::{Index, IndexMut};

extern "C" {
    static _trampoline: u8;
    static _trampoline_len: u32;
}

/// allign on
#[inline(always)]
fn align_on(t: usize, on: usize) -> usize {
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
pub struct Task {
    pub process_state: ProcessState,
    pub child: Vec<Pid>,
    pub parent: Option<Pid>,
    pub signal_actions: SignalActions,
    pub signal_queue: VecDeque<Signum>,
}

impl Task {
    pub fn new(parent: Option<Pid>, process_state: ProcessState) -> Self {
        Self {
            process_state,
            child: Vec::new(),
            parent,
            signal_actions: SignalActions([Sigaction::SigDfl; 32]),
            signal_queue: VecDeque::new(),
        }
    }

    pub fn unwrap_running_mut(&mut self) -> &mut UserProcess {
        match &mut self.process_state {
            ProcessState::Waiting(process, _) | ProcessState::Signaled(process) | ProcessState::Running(process) => {
                process
            }
            _ => panic!("WTF"),
        }
    }

    pub fn unwrap_running(&self) -> &UserProcess {
        match &self.process_state {
            ProcessState::Running(process) | ProcessState::Signaled(process) | ProcessState::Waiting(process, _) => {
                process
            }
            _ => panic!("WTF"),
        }
    }

    pub fn is_zombie(&self) -> bool {
        match self.process_state {
            ProcessState::Zombie(_) => true,
            _ => false,
        }
    }

    pub fn is_waiting(&self) -> bool {
        match self.process_state {
            ProcessState::Waiting(_, _) => true,
            _ => false,
        }
    }

    pub fn is_signaled(&self) -> bool {
        match self.process_state {
            ProcessState::Signaled(_) => true,
            _ => false,
        }
    }

    pub fn set_waiting(&mut self, waiting_state: WaitingState) {
        let uninit = unsafe { mem::uninitialized() };
        let prev = mem::replace(&mut self.process_state, uninit);
        let next = prev.set_waiting(waiting_state);
        let uninit = mem::replace(&mut self.process_state, next);
        mem::forget(uninit);
    }

    pub fn set_running(&mut self) {
        let uninit = unsafe { mem::uninitialized() };
        let prev = mem::replace(&mut self.process_state, uninit);
        let next = prev.set_running();
        let uninit = mem::replace(&mut self.process_state, next);
        mem::forget(uninit);
    }

    pub fn set_signaled(&mut self) {
        let uninit = unsafe { mem::uninitialized() };
        let prev = mem::replace(&mut self.process_state, uninit);
        let next = prev.set_signaled();
        let uninit = mem::replace(&mut self.process_state, next);
        mem::forget(uninit);
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

    /// check if there is pending sigals, and tricks the stack to execute it on return
    pub fn check_pending_signals(&mut self) {
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

        if !self.is_signaled() {
            if let Some(signum) = self.signal_queue.pop_front() {
                match self.signal_actions[signum] {
                    Sigaction::Handler(f) => {
                        let kernel_esp = self.unwrap_running().kernel_esp;
                        unsafe {
                            let cpu_state: *mut CpuState = kernel_esp as *mut CpuState;
                            let user_esp = &mut (*cpu_state).esp;

                            // push the current cpu_state on the user stack
                            push_esp(user_esp, *cpu_state);
                            // push the trampoline code on the user stack
                            push_buff_esp(user_esp, symbol_addr!(_trampoline) as *mut u8, _trampoline_len as usize);
                            // push the address of start of trampoline code stack on the user stack
                            let esp_trampoline = *user_esp;
                            push_esp(user_esp, esp_trampoline);

                            // set a fresh cpu state to execute the handler
                            let mut new_cpu_state = CpuState::new(*user_esp, f as u32);
                            new_cpu_state.eip = f as u32;

                            (*cpu_state) = new_cpu_state;
                            (*cpu_state).eip = f as u32;
                        }
                        self.set_signaled();
                    }
                    _ => {}
                }
            }
        }
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
            // skip the trampoline code
            (*cpu_state).esp += align_on(_trampoline_len as usize, 4) as u32;
            // get back the old cpu state and set it as the current cpu_state
            let old_cpu_state: CpuState = pop_esp(&mut (*cpu_state).esp);
            *cpu_state = old_cpu_state;

            self.set_running();
            // return current eax to keep it's value at the syscall return
            Ok((*cpu_state).registers.eax)
        }
    }
}

#[derive(Debug)]
pub enum WaitingState {
    /// The Process is sleeping until pit time >= u32 value
    Sleeping(u32),
    /// The Process is looking for the death of his child
    /// Set none for undefined PID or a child PID. Is followed by the status field
    ChildDeath(Option<Pid>, u32),
}

#[derive(Debug)]
pub enum ProcessState {
    /// The process is currently on running state
    Running(Box<UserProcess>),
    /// The process is currently waiting for something
    Waiting(Box<UserProcess>, WaitingState),
    /// The process is terminated and wait to deliver his testament to his father
    Signaled(Box<UserProcess>),
    /// The process is terminated and wait to deliver his testament to his father
    // TODO: Use bits 0..7 for normal exit(). Interpreted as i8 and set bit 31
    // TODO: Use bits 8..15 for signal exit. Interpreted as i8 and set bit 30
    Zombie(i32),
}

impl ProcessState {
    pub fn set_waiting(self, waiting_state: WaitingState) -> Self {
        match self {
            ProcessState::Running(p) => ProcessState::Waiting(p, waiting_state),
            ProcessState::Waiting(p, _) => ProcessState::Waiting(p, waiting_state),
            _ => panic!("Not handled by this feature"),
        }
    }
    pub fn set_running(self) -> Self {
        match self {
            ProcessState::Waiting(p, _) | ProcessState::Signaled(p) => ProcessState::Running(p),
            _ => panic!("already running"),
        }
    }
    pub fn set_signaled(self) -> Self {
        match self {
            ProcessState::Running(p) => ProcessState::Signaled(p),
            _ => panic!("already running"),
        }
    }
}
