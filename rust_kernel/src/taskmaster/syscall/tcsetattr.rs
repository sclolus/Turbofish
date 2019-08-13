//! tcsetattr syscall
use super::scheduler::SCHEDULER;
use super::SysResult;
use crate::terminal::TERMINAL;
use libc_binding::termios;

/// The tcsetattr() function shall set the parameters associated with
/// the terminal referred to by the open file descriptor fildes (an
/// open file descriptor associated with a terminal) from the termios
/// structure referenced by termios_p as follows:
///
/// If optional_actions is TCSANOW, the change shall occur
/// immediately.
///
/// If optional_actions is TCSADRAIN, the change shall occur after all
/// output written to fildes is transmitted. This function should be
/// used when changing parameters that affect output.
///
/// If optional_actions is TCSAFLUSH, the change shall occur after all
/// output written to fildes is transmitted, and all input so far
/// received but not read shall be discarded before the change is
/// made.
///
/// If the output baud rate stored in the termios structure pointed to
/// by termios_p is the zero baud rate, B0, the modem control lines
/// shall no longer be asserted. Normally, this shall disconnect the
/// line.
///
/// If the input baud rate stored in the termios structure pointed to
/// by termios_p is 0, the input baud rate given to the hardware is
/// the same as the output baud rate stored in the termios structure.
///
/// The tcsetattr() function shall return successfully if it was able
/// to perform any of the requested actions, even if some of the
/// requested actions could not be performed. It shall set all the
/// attributes that the implementation supports as requested and leave
/// all the attributes not supported by the implementation
/// unchanged. If no part of the request can be honored, it shall
/// return -1 and set errno to [EINVAL]. If the input and output baud
/// rates differ and are a combination that is not supported, neither
/// baud rate shall be changed. A subsequent call to tcgetattr() shall
/// return the actual state of the terminal device (reflecting both
/// the changes made and not made in the previous tcsetattr()
/// call). The tcsetattr() function shall not change the values found
/// in the termios structure under any circumstances.
///
/// The effect of tcsetattr() is undefined if the value of the termios
/// structure pointed to by termios_p was not derived from the result
/// of a call to tcgetattr() on fildes; an application should modify
/// only fields and flags defined by this volume of POSIX.1-2017
/// between the call to tcgetattr() and tcsetattr(), leaving all other
/// fields and flags unmodified.
///
/// No actions defined by this volume of POSIX.1-2017, other than a
/// call to tcsetattr(), a close of the last file descriptor in the
/// system associated with this terminal device, or an open of the
/// first file descriptor in the system associated with this terminal
/// device (using the O_TTY_INIT flag if it is non-zero and the device
/// is not a pseudo-terminal), shall cause any of the terminal
/// attributes defined by this volume of POSIX.1-2017 to change.
///
/// If tcsetattr() is called from a process which is a member of a
/// background process group on a fildes associated with its
/// controlling terminal:
///
/// If the calling thread is blocking SIGTTOU signals or the process
/// is ignoring SIGTTOU signals, the operation completes normally and
/// no signal is sent.
///
/// Otherwise, a SIGTTOU signal shall be sent to the process group.
// TODO: file descriptor argument
pub fn sys_tcsetattr(
    _fildes: i32,
    optional_actions: u32,
    termios_p: *const termios,
) -> SysResult<u32> {
    unpreemptible_context!({
        // TODO: change this 1
        {
            let scheduler = SCHEDULER.lock();
            let v = scheduler
                .current_task()
                .unwrap_process()
                .get_virtual_allocator();

            // Check if pointer exists in user virtual address space
            v.check_user_ptr(termios_p)?;
        }
        unsafe {
            TERMINAL
                .as_mut()
                .unwrap()
                .get_line_discipline(1)
                .tcsetattr(optional_actions, &*termios_p);
        }
    });
    Ok(0)
}
