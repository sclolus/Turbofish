//! This file contains signal interface

use super::process::CpuState;
use super::SysResult;

use alloc::collections::vec_deque::VecDeque;
use bit_field::BitField;
use core::convert::TryFrom;
use core::mem;
use core::mem::{size_of, transmute};
use core::ops::BitOr;
use core::ops::{Index, IndexMut};

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

#[allow(dead_code)]
pub enum DefaultAction {
    Abort,
    Terminate,
    Ignore,
    Stop,
    Continue,
}

#[allow(dead_code)]
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

type FunctionAddress = usize;

#[derive(Copy, Clone, Debug, Default)]
#[repr(transparent)]
pub struct SaMask(u32);

impl SaMask {
    fn contains(&self, signum: Signum) -> bool {
        self.0.get_bit(signum as u32 as usize)
    }
}

impl From<Signum> for SaMask {
    fn from(s: Signum) -> Self {
        Self(1 << s as u32)
    }
}

impl BitOr for SaMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

const SIG_DFL: usize = 0;
const SIG_IGN: usize = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct StructSigaction {
    pub sa_handler: usize,
    // TODO: Must be an union with sa_handler
    // sa_sigaction: extern "C" fn(int, siginfo_t *, void *),
    pub sa_mask: SaMask,
    pub sa_flags: u32,
    pub sa_restorer: usize,
}

// impl StructSigaction {
//     fn is_ignored(&self) -> bool {
//         self.sa_handler == SIG_IGN
//     }
//     fn is_default(&self) -> bool {
//         self.sa_handler == SIG_DFL
//     }
// }

impl Default for StructSigaction {
    fn default() -> Self {
        Self { sa_handler: SIG_DFL, sa_mask: Default::default(), sa_flags: 0, sa_restorer: 0 }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SignalActions(pub [StructSigaction; 32]);

impl IndexMut<Signum> for SignalActions {
    fn index_mut(&mut self, index: Signum) -> &mut StructSigaction {
        &mut self.0[index as usize]
    }
}

impl Index<Signum> for SignalActions {
    type Output = StructSigaction;
    fn index(&self, index: Signum) -> &Self::Output {
        &self.0[index as usize]
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SignalStatus {
    Handled(Signum),
    Deadly(Signum),
}

#[derive(Debug)]
pub struct SignalInterface {
    pub signal_actions: SignalActions,
    pub signal_queue: VecDeque<Signum>,
    pub current_sa_mask: SaMask,
}

impl SignalInterface {
    /// Create a new signal Inteface
    pub fn new() -> Self {
        Self {
            signal_actions: SignalActions([Default::default(); 32]),
            signal_queue: VecDeque::new(),
            current_sa_mask: Default::default(),
        }
    }

    /// Check all pendings signals: Sort them if necessary and return the first signal will be launched
    pub fn check_pending_signals(&mut self) -> Option<SignalStatus> {
        let signum = *self.signal_queue.get(0)?;

        if self.current_sa_mask.contains(signum) {
            return None;
        }
        let sigaction = self.signal_actions[signum];
        match sigaction.sa_handler {
            SIG_DFL => {
                use DefaultAction::*;
                match signal_default_action(signum) {
                    Abort => Some(SignalStatus::Deadly(signum)),
                    Terminate => Some(SignalStatus::Deadly(signum)),
                    Ignore => unimplemented!(),
                    Continue => unimplemented!(),
                    Stop => unimplemented!(),
                }
            }
            SIG_IGN => unimplemented!(),
            _ => Some(SignalStatus::Handled(signum)),
        }
    }

    /// Apply all the checked signals: Make signals frames if no deadly. Returns DEADLY directive or first signal
    pub fn apply_pending_signals(&mut self, process_context_ptr: u32) -> Option<SignalStatus> {
        let signum = *self.signal_queue.get(0)?;

        if self.current_sa_mask.contains(signum) {
            return None;
        }
        let sigaction = self.signal_actions[signum];
        match sigaction.sa_handler {
            SIG_DFL => {
                use DefaultAction::*;
                match signal_default_action(signum) {
                    Abort => Some(SignalStatus::Deadly(signum)),
                    Terminate => Some(SignalStatus::Deadly(signum)),
                    Ignore => unimplemented!(),
                    Continue => unimplemented!(),
                    Stop => unimplemented!(),
                }
            }
            SIG_IGN => unimplemented!(),
            _ => {
                self.exec_signal_handler(signum, process_context_ptr, &sigaction);
                None
            }
        }
    }

    /// Acknowledge end of signal execution, pop the first internal signal and a restore context form the signal frame.
    pub fn terminate_pending_signal(&mut self, process_context_ptr: u32) {
        /// helper to pop on the stack
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
        let cpu_state = process_context_ptr as *mut CpuState;
        unsafe {
            // eprintln!("sigreturn");
            // dbg_hex!(*cpu_state);
            // skip the trampoline code
            (*cpu_state).esp += align_on(_trampoline_len as usize, 4) as u32;
            // get back the old cpu state and set it as the current cpu_state
            let _signum: u32 = pop_esp(&mut (*cpu_state).esp);
            self.current_sa_mask = pop_esp(&mut (*cpu_state).esp);
            let old_cpu_state: CpuState = pop_esp(&mut (*cpu_state).esp);
            // dbg_hex!(old_cpu_state);
            *cpu_state = old_cpu_state;

            // return current eax to keep it's value at the syscall return
        }
        // self.signal_queue
        //     .pop_front()
        //     .expect("Unexpected empty signal queue");
    }

    /// Register a new handler for a specified Signum
    pub fn new_handler(&mut self, signum: Signum, sigaction: &StructSigaction) -> SysResult<u32> {
        let former = mem::replace(&mut self.signal_actions[signum], *sigaction);
        Ok(former.sa_handler as u32)
    }

    /// Register a new signal
    pub fn new_signal(&mut self, signum: Signum) -> SysResult<u32> {
        self.signal_queue.try_reserve(1)?;
        self.signal_queue.push_back(signum);
        Ok(0)
    }

    fn exec_signal_handler(&mut self, signum: Signum, kernel_esp: u32, sigaction: &StructSigaction) {
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
        unsafe {
            let cpu_state: *mut CpuState = kernel_esp as *mut CpuState;
            // dbg_hex!(*cpu_state);

            let mut user_esp = (*cpu_state).esp;
            // push the current cpu_state on the user stack
            push_esp(&mut user_esp, *cpu_state);
            push_esp(&mut user_esp, self.current_sa_mask);
            // push the trampoline code on the user stack
            push_buff_esp(&mut user_esp, symbol_addr!(_trampoline) as *mut u8, _trampoline_len as usize);
            // push the address of start of trampoline code stack on the user stack
            let esp_trampoline = user_esp;
            push_esp(&mut user_esp, signum as u32);
            push_esp(&mut user_esp, esp_trampoline);

            (*cpu_state).eip = sigaction.sa_handler as u32;
            (*cpu_state).esp = user_esp;
            // dbg_hex!(*cpu_state);
        }
        self.current_sa_mask = self.current_sa_mask | sigaction.sa_mask | SaMask::from(signum);
        self.signal_queue.pop_front().expect("Unexpected empty signal queue");
    }
}
