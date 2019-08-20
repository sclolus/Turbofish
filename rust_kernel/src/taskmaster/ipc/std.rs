//! This file contains all the stuff about std File Descriptors

use super::SysResult;

use super::FileOperation;
use super::IpcResult;
use super::Mode;

mod stdin;
pub use stdin::Stdin;
mod stdout;
pub use stdout::Stdout;
mod stderr;
pub use stderr::Stderr;
