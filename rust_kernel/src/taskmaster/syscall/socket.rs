//! This file contains the description of the socketcall syscall

use super::scheduler::auto_preempt;
use super::IpcResult;
use super::SysResult;

use super::scheduler::{Scheduler, SCHEDULER};

pub type SocketArgsPtr = *const u8;

use super::vfs::{Path, VFS};
use core::mem::transmute;
use libc_binding::c_char;
use libc_binding::{Errno, PATH_MAX};

const PATH_MAX_USIZE: usize = PATH_MAX as usize;

use sync::DeadMutexGuard;

use core::convert::{TryFrom, TryInto};
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
        pub enum $name {
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
                    _ => Err(Errno::EINVAL),
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
     struct $name:tt([$type:tt; $q:ident]);
    ) => {
        #[$main_doc]
        $(#[$e])*
        struct $name([$type; $q]);

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

visible_byte_array!(
    /// Unix pathname
    #[derive(Copy, Clone)]
    struct SockPathname([c_char; PATH_MAX_USIZE]);
);

impl SockPathname {
    pub fn as_str(&self) -> SysResult<&str> {
        let end = self.0.iter().position(|c| *c == 0).ok_or(Errno::EINVAL)?;
        core::str::from_utf8(unsafe { transmute::<&[i8], &[u8]>(&self.0[0..end]) })
            .map_err(|_| Errno::EINVAL)
    }
}

/// This is the basic structure for exchanging packet with UNIX socket
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct SockaddrUnix {
    /// TypeOf Socket
    sun_family: SunFamily,
    /// Unix pathname
    sun_path: SockPathname,
}

/// They are differents types of sockaddr
#[derive(Debug)]
pub enum Sockaddr {
    /// UNIX socket
    Unix(&'static SockaddrUnix),
}

impl TryFrom<SockaddrUnix> for Path {
    type Error = Errno;
    fn try_from(unix_sockaddr: SockaddrUnix) -> Result<Self, Self::Error> {
        Ok(Path::try_from(unix_sockaddr.sun_path.as_str()?)?)
    }
}

impl TryInto<SockPathname> for Path {
    type Error = Errno;
    fn try_into(self) -> Result<SockPathname, Self::Error> {
        let mut res = SockPathname([0; PATH_MAX_USIZE]);
        self.write_path_in_buffer(&mut res.0[..])?;
        Ok(res)
    }
}

impl TryFrom<Sockaddr> for Path {
    type Error = Errno;
    fn try_from(sockaddr: Sockaddr) -> Result<Self, Self::Error> {
        Ok(match sockaddr {
            Sockaddr::Unix(unix_sockaddr) => Path::try_from(*unix_sockaddr)?,
        })
    }
}

/// TryFrom boilerplate for Sockaddr
impl core::convert::TryFrom<(&DeadMutexGuard<'_, AddressSpace>, *const u8, usize)> for Sockaddr {
    type Error = Errno;
    fn try_from(
        arg: (&DeadMutexGuard<AddressSpace>, *const u8, usize),
    ) -> Result<Self, Self::Error> {
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
                                .ok_or(Errno::EINVAL)?,
                        ))
                    }
                } else {
                    Err(Errno::EINVAL)
                }
            }
        }
    }
}

/// Main syscall interface dispatcher
pub fn sys_socketcall(call_type: u32, args: SocketArgsPtr) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let v = scheduler
            .current_thread_mut()
            .unwrap_process_mut()
            .get_virtual_allocator();

        let call: CallType = call_type.try_into()?;

        use CallType::*;
        match call {
            SysSocket => {
                dbg!("socket");
                v.check_user_ptr::<SocketArgs>(args as *const SocketArgs)?;
                drop(v);
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
                dbg!("bind");
                v.check_user_ptr::<BindArgs>(args as *const BindArgs)?;
                let BindArgs {
                    socket_fd,
                    addr,
                    addr_len,
                } = unsafe { *(args as *const BindArgs) };
                let sockaddr = (&v, addr as *const u8, addr_len as usize).try_into()?;
                drop(v);
                bind(&mut scheduler, socket_fd as i32, sockaddr)
            }
            SysConnect => {
                dbg!("connect");
                v.check_user_ptr::<ConnectArgs>(args as *const ConnectArgs)?;
                let ConnectArgs {
                    socket_fd,
                    addr,
                    addr_len,
                } = unsafe { *(args as *const ConnectArgs) };
                let sockaddr = (&v, addr as *const u8, addr_len as usize).try_into()?;
                drop(v);
                connect(&mut scheduler, socket_fd as i32, sockaddr)
            }
            SysListen => {
                dbg!("listen");
                v.check_user_ptr::<ListenArgs>(args as *const ListenArgs)?;
                drop(v);
                let ListenArgs { socket_fd, backlog } = unsafe { *(args as *const ListenArgs) };
                listen(&mut scheduler, socket_fd as i32, backlog as i32)
            }
            SysAccept => {
                dbg!("accept");
                v.check_user_ptr::<AcceptArgs>(args as *const AcceptArgs)?;
                drop(v);
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
                dbg!("send");
                v.check_user_ptr::<SendArgs>(args as *const SendArgs)?;
                let SendArgs {
                    socket_fd,
                    buf,
                    len,
                    flags,
                } = unsafe { *(args as *const SendArgs) };
                let mem = v.make_checked_slice(buf as *const u8, len as usize)?;
                drop(v);
                send(&mut scheduler, socket_fd as i32, mem, flags)
            }
            SysRecv => {
                dbg!("recv");
                v.check_user_ptr::<RecvArgs>(args as *const RecvArgs)?;
                let RecvArgs {
                    socket_fd,
                    buf,
                    len,
                    flags,
                } = unsafe { *(args as *const RecvArgs) };
                let mem = v.make_checked_mut_slice(buf as *mut u8, len as usize)?;
                drop(v);
                recv_from(
                    &mut scheduler,
                    socket_fd as i32,
                    mem,
                    flags,
                    None,
                    core::ptr::null_mut(),
                )
            }
            SysSendTo => {
                dbg!("sendto");
                v.check_user_ptr::<SendToArgs>(args as *const SendToArgs)?;
                let SendToArgs {
                    socket_fd,
                    buf,
                    len,
                    flags,
                    dst_addr,
                    addr_len,
                } = unsafe { *(args as *const SendToArgs) };
                let mem = v.make_checked_slice(buf as *const u8, len as usize)?;
                let sockaddr_opt: Option<Sockaddr> = if dst_addr != 0x0 {
                    Some((&v, dst_addr as *const u8, addr_len as usize).try_into()?)
                } else {
                    None
                };
                drop(v);
                send_to(&mut scheduler, socket_fd as i32, mem, flags, sockaddr_opt)
            }
            SysRecvFrom => {
                dbg!("recvfrom");
                v.check_user_ptr::<RecvFromArgs>(args as *const RecvFromArgs)?;
                let RecvFromArgs {
                    socket_fd,
                    buf,
                    len,
                    flags,
                    src_addr,
                    addr_len,
                } = unsafe { *(args as *const RecvFromArgs) };
                let src_addr = {
                    let src_addr = src_addr as *mut SockaddrUnix;
                    if src_addr.is_null() {
                        None
                    } else {
                        Some(v.make_checked_ref_mut(src_addr)?)
                    }
                };
                drop(v);
                let mem = unsafe { slice::from_raw_parts_mut(buf as *mut u8, len as usize) };
                // UNSAFE pointers are passed to recv_from(). The syscall MUST check them before filling
                recv_from(
                    &mut scheduler,
                    socket_fd as i32,
                    mem,
                    flags,
                    src_addr,
                    addr_len as *mut SockLen,
                )
            }
            SysShutdown => {
                dbg!("shutdown");
                v.check_user_ptr::<ShutdownArgs>(args as *const ShutdownArgs)?;
                drop(v);
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
    scheduler: &mut Scheduler,
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
    let tg = scheduler.current_thread_group_mut();
    let fd_interface = &mut tg
        .thread_group_state
        .unwrap_running_mut()
        .file_descriptor_interface;
    fd_interface.open_socket(domain, socket_type)
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

fn bind(scheduler: &mut Scheduler, socket_fd: i32, sockaddr: Sockaddr) -> SysResult<u32> {
    println!("{:?}: {:?} {:?}", function!(), socket_fd, sockaddr);

    let tg = scheduler.current_thread_group_mut();
    let creds = &tg.credentials;
    let cwd = &tg.cwd;
    let fd_interface = &mut tg
        .thread_group_state
        .unwrap_running_mut()
        .file_descriptor_interface;
    let file_operation = &mut fd_interface.get_file_operation(socket_fd as u32)?;
    let path = sockaddr.try_into()?;
    file_operation.bind(&cwd, creds, path)
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

// fn recv(_scheduler: &mut Scheduler, socket_fd: i32, buf: &mut [u8], flags: u32) -> SysResult<u32> {
//     println!(
//         "{:?}: {:?} {:?} {:?}",
//         function!(),
//         socket_fd,
//         unsafe { core::str::from_utf8_unchecked(buf) },
//         flags
//     );
//     Ok(0)
// }

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
    scheduler: &mut Scheduler,
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
    let tg = scheduler.current_thread_group_mut();
    let cwd = &tg.cwd;
    let fd_interface = &mut tg
        .thread_group_state
        .unwrap_running_mut()
        .file_descriptor_interface;
    let file_operation = &mut fd_interface.get_file_operation(socket_fd as u32)?;
    let path = match sockaddr_opt {
        Some(sockaddr) => Some(VFS.lock().resolve_path(cwd, &sockaddr.try_into()?)?),
        None => None,
    };
    file_operation.send_to(buf, flags, path)?;
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
    scheduler: &mut Scheduler,
    socket_fd: i32,
    buf: &mut [u8],
    flags: u32,
    src_addr: Option<&mut SockaddrUnix>,
    addr_len: *mut SockLen,
) -> SysResult<u32> {
    println!(
        "{:?}: {:?} {:?} {:?} {:?} {:?}",
        function!(),
        socket_fd,
        buf,
        flags,
        src_addr,
        addr_len
    );
    let tg = scheduler.current_thread_group_mut();
    let _cwd = &tg.cwd;
    let fd_interface = &mut tg
        .thread_group_state
        .unwrap_running_mut()
        .file_descriptor_interface;
    let file_operation = &mut fd_interface.get_file_operation(socket_fd as u32)?;
    loop {
        match file_operation.recv_from(buf, flags)? {
            IpcResult::Done((readen_bytes, sender_path)) => {
                if let Some(src_addr) = src_addr {
                    if let Some(sender_path) = sender_path {
                        *src_addr = SockaddrUnix {
                            sun_family: SunFamily::AfUnix,
                            sun_path: sender_path.try_into()?,
                        }
                    }
                }
                return Ok(readen_bytes);
            }
            IpcResult::Wait(_, _) => {
                // UNLOCK SCHEDULER
                let _ret = auto_preempt()?;
            }
        }
    }
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
