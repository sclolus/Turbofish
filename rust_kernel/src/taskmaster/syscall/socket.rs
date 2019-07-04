//! This file contains the description of the socketcall syscall

use super::SysResult;

use super::scheduler::{Scheduler, SCHEDULER};

pub type SocketArgsPtr = *const u8;

use errno::Errno;

use alloc::sync::Arc;
use core::convert::TryInto;
use core::slice;

use crate::memory::AddressSpace;

/// Implements a new C style enum with his try_from boilerplate
macro_rules! safe_convertible_enum {
    (#[$main_doc:meta]
     #[$derive:meta]
     #[repr($primitive_type:tt)]
     enum $name:ident {
         $(
             #[$doc:meta]
             $variant:ident = $value:expr,
         )*
     }) => {
        #[$main_doc]
        #[$derive]
        #[repr($primitive_type)]
        enum $name {
            $(
                #[$doc]
                $variant = $value,
            )*
        }

        impl core::convert::TryFrom<$primitive_type> for $name {
            type Error = Errno;
            fn try_from(n: $primitive_type) -> Result<Self, Self::Error> {
                use $name::*;
                match n {
                    $($value => Ok($variant),)*
                    _ => Err(Errno::Einval),
                }
            }
        }
    }
}

/// A simple macro to handle raw fields
macro_rules! raw_deferencing_struct {
    (#[$main_doc:meta]
     $(#[$e:meta])*
     struct $name:tt {
         $(
             #[$doc:meta]
             $field:ident: $type:ty,
         )*
     }) => {
        #[$main_doc]
        $(#[$e])*
        struct $name {
            $(
                #[$doc]
                $field: u32,
            )*
        }
    }
}

/// Implements the debug boilerplate to raw string based on byte array from C style
macro_rules! visible_byte_array {
    (#[$main_doc:meta]
     $(#[$e:meta])*
     struct $name:tt([u8; $q:ident]);
    ) => {
        #[$main_doc]
        $(#[$e])*
        struct $name([u8; $q]);

        impl core::fmt::Debug for $name {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
             unsafe {
             let ptr = self as *const _ as *const u8;
                 let mut i = 0;
                 while (*ptr.offset(i as isize)) != 0 {
                     i += 1;
                 }
                 let slice: &[u8] = core::slice::from_raw_parts(ptr, i); // Make slice of u8 (&[u8])
                 write!(f, "{}", core::str::from_utf8_unchecked(slice))  // Make str slice (&[str]) with &[u8]
             }
        }}
    }
}

safe_convertible_enum!(
    /// This list contains the sockets associated function types
    #[derive(Debug, Copy, Clone)]
    #[repr(u32)]
    enum CallType {
        /// Create an endpoint for communication
        SysSocket = 1,
        /// Bind a name to a socket
        SysBind = 2,
        /// Initiate a connection on a socket. Client connection-oriented
        SysConnect = 3,
        /// Listen for connections on a socket. Server connection-oriented
        SysListen = 4,
        /// Accept a connection on a socket. Server connection-oriented
        SysAccept = 5,
        /// Send a message on a socket. Similar to write with flags. connection-oriented
        SysSend = 9,
        /// Receive a message from a socket. Similar to read with flags. connection-oriented
        SysRecv = 10,
        /// Send a message on a socket. The destination address is specified. connectionless
        SysSendTo = 11,
        /// Receive a message from a socket. The source address is specified. connectionless
        SysRecvFrom = 12,
        /// Shut down part of a full-duplex connection. connection-oriented
        SysShutdown = 13,
    }
);

safe_convertible_enum!(
    /// For the moment, we just handle UNIX basic sockets, not AF_INET or everything else
    #[derive(Debug, Copy, Clone)]
    #[repr(u16)]
    enum SunFamily {
        /// UNIX socket
        AfUnix = 1,
    }
);

const UNIX_PATHNAME_MAXSIZE: usize = 108;

visible_byte_array!(
    /// Unix pathname
    #[derive(Copy, Clone)]
    struct SockPathname([u8; UNIX_PATHNAME_MAXSIZE]);
);

/// This is the basic structure for exchanging packet with UNIX socket
#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct SockaddrUnix {
    /// TypeOf Socket
    sun_family: SunFamily,
    /// Unix pathname
    sun_path: SockPathname,
}

/// They are differents types of sockaddr
#[derive(Debug)]
enum Sockaddr {
    /// UNIX socket
    Unix(&'static SockaddrUnix),
}

/// TryFrom boilerplate for Sockaddr
impl core::convert::TryFrom<(&Arc<AddressSpace>, *const u8, usize)> for Sockaddr {
    type Error = Errno;
    fn try_from(arg: (&Arc<AddressSpace>, *const u8, usize)) -> Result<Self, Self::Error> {
        arg.0
            .check_user_ptr::<SunFamily>(arg.1 as *const SunFamily)?;
        let raw_family = unsafe { *(arg.1 as *const u16) };
        match raw_family.try_into()? {
            SunFamily::AfUnix => {
                if arg.2 == core::mem::size_of::<SockaddrUnix>() {
                    arg.0
                        .check_user_ptr::<SockaddrUnix>(arg.1 as *const SockaddrUnix)?;
                    unsafe {
                        Ok(Sockaddr::Unix(
                            (arg.1 as *const SockaddrUnix)
                                .as_ref()
                                .ok_or(Errno::Einval)?,
                        ))
                    }
                } else {
                    Err(Errno::Einval)
                }
            }
        }
    }
}

/// Main syscall interface dispatcher
pub fn sys_socketcall(call_type: u32, args: SocketArgsPtr) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let v = &scheduler
            .current_task_mut()
            .unwrap_process_mut()
            .virtual_allocator;

        let call: CallType = call_type.try_into()?;

        use CallType::*;
        match call {
            SysSocket => {
                v.check_user_ptr::<SocketArgs>(args as *const SocketArgs)?;
                let SocketArgs {
                    domain,
                    socket_type,
                    protocol,
                } = unsafe { *(args as *const SocketArgs) };
                socket(
                    &mut scheduler,
                    domain.try_into()?,
                    socket_type.try_into()?,
                    protocol,
                )
            }
            SysBind => {
                v.check_user_ptr::<BindArgs>(args as *const BindArgs)?;
                let BindArgs {
                    socket_fd,
                    addr,
                    addr_len,
                } = unsafe { *(args as *const BindArgs) };
                let sockaddr = (v, addr as *const u8, addr_len as usize).try_into()?;
                bind(&mut scheduler, socket_fd as i32, sockaddr)
            }
            SysConnect => {
                v.check_user_ptr::<ConnectArgs>(args as *const ConnectArgs)?;
                let ConnectArgs {
                    socket_fd,
                    addr,
                    addr_len,
                } = unsafe { *(args as *const ConnectArgs) };
                let sockaddr = (v, addr as *const u8, addr_len as usize).try_into()?;
                connect(&mut scheduler, socket_fd as i32, sockaddr)
            }
            SysListen => {
                v.check_user_ptr::<ListenArgs>(args as *const ListenArgs)?;
                let ListenArgs { socket_fd, backlog } = unsafe { *(args as *const ListenArgs) };
                listen(&mut scheduler, socket_fd as i32, backlog as i32)
            }
            SysAccept => {
                v.check_user_ptr::<AcceptArgs>(args as *const AcceptArgs)?;
                let AcceptArgs {
                    socket_fd,
                    addr,
                    addr_len,
                } = unsafe { *(args as *const AcceptArgs) };
                // UNSAFE pointers are passed to accept(). The syscall MUST check them before filling
                accept(
                    &mut scheduler,
                    socket_fd as i32,
                    addr as *mut u8,
                    addr_len as *mut SockLen,
                )
            }
            SysSend => {
                v.check_user_ptr::<SendArgs>(args as *const SendArgs)?;
                let SendArgs {
                    socket_fd,
                    buf,
                    len,
                    flags,
                } = unsafe { *(args as *const SendArgs) };
                let mem = unsafe { slice::from_raw_parts(buf as *const u8, len as usize) };
                v.check_user_ptr_with_len::<u8>(mem.as_ptr(), mem.len())?;
                send(&mut scheduler, socket_fd as i32, mem, flags)
            }
            SysRecv => {
                v.check_user_ptr::<RecvArgs>(args as *const RecvArgs)?;
                let RecvArgs {
                    socket_fd,
                    buf,
                    len,
                    flags,
                } = unsafe { *(args as *const RecvArgs) };
                let mem = unsafe { slice::from_raw_parts_mut(buf as *mut u8, len as usize) };
                v.check_user_ptr_with_len::<u8>(mem.as_ptr(), mem.len())?;
                recv(&mut scheduler, socket_fd as i32, mem, flags)
            }
            SysSendTo => {
                v.check_user_ptr::<SendToArgs>(args as *const SendToArgs)?;
                let SendToArgs {
                    socket_fd,
                    buf,
                    len,
                    flags,
                    dst_addr,
                    addr_len,
                } = unsafe { *(args as *const SendToArgs) };
                let mem = unsafe { slice::from_raw_parts(buf as *const u8, len as usize) };
                v.check_user_ptr_with_len::<u8>(mem.as_ptr(), mem.len())?;
                let sockaddr_opt: Option<Sockaddr> = if dst_addr != 0x0 {
                    Some((v, dst_addr as *const u8, addr_len as usize).try_into()?)
                } else {
                    None
                };
                send_to(&mut scheduler, socket_fd as i32, mem, flags, sockaddr_opt)
            }
            SysRecvFrom => {
                v.check_user_ptr::<RecvFromArgs>(args as *const RecvFromArgs)?;
                let RecvFromArgs {
                    socket_fd,
                    buf,
                    len,
                    flags,
                    src_addr,
                    addr_len,
                } = unsafe { *(args as *const RecvFromArgs) };
                let mem = unsafe { slice::from_raw_parts_mut(buf as *mut u8, len as usize) };
                // UNSAFE pointers are passed to recv_from(). The syscall MUST check them before filling
                recv_from(
                    &mut scheduler,
                    socket_fd as i32,
                    mem,
                    flags,
                    src_addr as *mut u8,
                    addr_len as *mut SockLen,
                )
            }
            SysShutdown => {
                v.check_user_ptr::<ShutdownArgs>(args as *const ShutdownArgs)?;
                let ShutdownArgs { socket_fd, how } = unsafe { *(args as *const ShutdownArgs) };
                shutdown(&mut scheduler, socket_fd as i32, how)
            }
        }
    })
}

safe_convertible_enum!(
    /// Same thing than SunFamily
    #[derive(Debug, Copy, Clone)]
    #[repr(u32)]
    enum Domain {
        /// Local communication. Basic unix sockets
        AfUnix = 1,
    }
);

safe_convertible_enum!(
    /// Connection mode
    #[derive(Debug, Copy, Clone)]
    #[repr(u32)]
    enum SocketType {
        /// Connection-oriented
        SockStream = 1,
        /// Connectionless, unreliable messages of a fixed maximum length
        SockDgram = 2,
    }
);

raw_deferencing_struct!(
    /// Arguments for socket() function
    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    struct SocketArgs {
        /// The  domain argument specifies a communication domain
        domain: Domain,
        /// The socket has the indicated type, which specifies the communication semantics
        socket_type: SocketType,
        /// Dont worry. can be 0
        protocol: u32,
    }
);

type SockLen = usize;

fn socket(
    _scheduler: &mut Scheduler,
    domain: Domain,
    socket_type: SocketType,
    protocol: u32,
) -> SysResult<u32> {
    println!(
        "{:?}: {:?} {:?} {:?}",
        function!(),
        domain,
        socket_type,
        protocol
    );
    Ok(3)
}

raw_deferencing_struct!(
    /// Arguments for bind() function
    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    struct BindArgs {
        /// Associated socket file decriptor
        socket_fd: i32,
        /// Sockaddr strucure pointer
        addr: *const Sockaddr,
        /// Length of previous structure
        addr_len: SockLen,
    }
);

fn bind(_scheduler: &mut Scheduler, socket_fd: i32, sockaddr: Sockaddr) -> SysResult<u32> {
    println!("{:?}: {:?} {:?}", function!(), socket_fd, sockaddr);
    Ok(0)
}

raw_deferencing_struct!(
    /// Arguments for connect() function
    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    struct ConnectArgs {
        /// Associated socket file decriptor
        socket_fd: i32,
        /// Sockaddr strucure pointer
        addr: *const Sockaddr,
        /// Length of previous structure
        addr_len: SockLen,
    }
);

fn connect(_scheduler: &mut Scheduler, socket_fd: i32, sockaddr: Sockaddr) -> SysResult<u32> {
    println!("{:?}: {:?} {:?}", function!(), socket_fd, sockaddr);
    Ok(0)
}

raw_deferencing_struct!(
    /// Arguments for listen() function
    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    struct ListenArgs {
        /// Associated socket file decriptor
        socket_fd: i32,
        /// Maximum length to which the queue of pending connections
        backlog: i32,
    }
);

fn listen(_scheduler: &mut Scheduler, socket_fd: i32, backlog: i32) -> SysResult<u32> {
    println!("{:?}: {:?} {:?}", function!(), socket_fd, backlog);
    Ok(0)
}

raw_deferencing_struct!(
    /// Arguments for accept() function
    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    struct AcceptArgs {
        /// Associated socket file decriptor
        socket_fd: i32,
        /// Sockaddr strucure pointer. Ths syscall must fill this structure if NON-NULL
        addr: *mut Sockaddr,
        /// Length of previous structure. The syscall must specify the length of sockaddr
        addr_len: *mut SockLen,
    }
);

// This function cannot be completely safe by nature of theses functionalities.
fn accept(
    _scheduler: &mut Scheduler,
    socket_fd: i32,
    sockaddr: *mut u8,
    sockaddr_len: *mut SockLen,
) -> SysResult<u32> {
    println!(
        "{:?}: {:?} {:?} {:?}",
        function!(),
        socket_fd,
        sockaddr,
        sockaddr_len
    );
    Ok(0)
}

raw_deferencing_struct!(
    /// Arguments for send() function
    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    struct SendArgs {
        /// Associated socket file decriptor
        socket_fd: i32,
        /// buffer to send
        buf: *const u8,
        /// Length of the buffer
        len: usize,
        /// Optional flags
        flags: u32,
    }
);

fn send(_scheduler: &mut Scheduler, socket_fd: i32, buf: &[u8], flags: u32) -> SysResult<u32> {
    println!(
        "{:?}: {:?} {:?} {:?}",
        function!(),
        socket_fd,
        unsafe { core::str::from_utf8_unchecked(buf) },
        flags
    );
    Ok(0)
}

raw_deferencing_struct!(
    /// Arguments for recv() function
    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    struct RecvArgs {
        /// Associated socket file decriptor
        socket_fd: i32,
        /// buffer to read
        buf: *mut u8,
        /// Length of the buffer
        len: usize,
        /// Optional flags
        flags: u32,
    }
);

fn recv(_scheduler: &mut Scheduler, socket_fd: i32, buf: &mut [u8], flags: u32) -> SysResult<u32> {
    println!(
        "{:?}: {:?} {:?} {:?}",
        function!(),
        socket_fd,
        unsafe { core::str::from_utf8_unchecked(buf) },
        flags
    );
    Ok(0)
}

raw_deferencing_struct!(
    /// Arguments for send_to() function
    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    struct SendToArgs {
        /// Associated socket file decriptor
        socket_fd: i32,
        /// buffer to send
        buf: *const u8,
        /// Length of the buffer
        len: usize,
        /// Optional flags
        flags: u32,
        /// Sockaddr strucure pointer
        dst_addr: *const Sockaddr,
        /// Length of previous structure
        addr_len: SockLen,
    }
);

fn send_to(
    _scheduler: &mut Scheduler,
    socket_fd: i32,
    buf: &[u8],
    flags: u32,
    sockaddr_opt: Option<Sockaddr>,
) -> SysResult<u32> {
    println!(
        "{:?}: {:?} {:?} {:?} {:?}",
        function!(),
        socket_fd,
        unsafe { core::str::from_utf8_unchecked(buf) },
        flags,
        sockaddr_opt
    );
    Ok(0)
}

raw_deferencing_struct!(
    /// Arguments for recv_from() function
    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    struct RecvFromArgs {
        /// Associated socket file decriptor
        socket_fd: i32,
        /// buffer to read
        buf: *mut u8,
        /// Length of the buffer
        len: usize,
        /// Optional flags
        flags: u32,
        /// Sockaddr strucure pointer. Ths syscall must fill this structure if NON-NULL
        src_addr: *mut Sockaddr,
        /// Length of previous structure. The syscall must specify the length of sockaddr
        addr_len: *mut SockLen,
    }
);

// This function cannot be completely safe by nature of theses functionalities.
fn recv_from(
    _scheduler: &mut Scheduler,
    socket_fd: i32,
    buf: &mut [u8],
    flags: u32,
    src_addr: *mut u8,
    addr_len: *mut SockLen,
) -> SysResult<u32> {
    println!(
        "{:?}: {:?} {:?} {:?} {:?} {:?}",
        function!(),
        socket_fd,
        unsafe { core::str::from_utf8_unchecked(buf) },
        flags,
        src_addr,
        addr_len
    );
    Ok(0)
}

raw_deferencing_struct!(
    /// Arguments for shutdown() function
    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    struct ShutdownArgs {
        /// Associated socket file decriptor
        socket_fd: i32,
        /// How the shutdown must be done ?
        how: u32,
    }
);

fn shutdown(_scheduler: &mut Scheduler, socket_fd: i32, how: u32) -> SysResult<u32> {
    println!("{:?}: {:?} {:?}", function!(), socket_fd, how);
    Ok(0)
}
