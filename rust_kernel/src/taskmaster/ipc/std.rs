//! This file contains all the stuff about std File Descriptors

use super::SysResult;

use super::IpcResult;
use super::KernelFileDescriptor;
use super::Mode;

mod stdin;
pub use stdin::Stdin;
mod stdout;
pub use stdout::Stdout;
mod stderr;
pub use stderr::Stderr;
