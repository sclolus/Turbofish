//! This file contains the task manager

mod cpu_isr;
// mod ipc;
mod process;
#[macro_use]
mod scheduler;
pub mod drivers;
pub use drivers::{Driver, FileOperation};
mod fd_interface;
mod safe_ffi;
mod signal_interface;
mod syscall;
mod tests;
mod thread;
mod thread_group;
pub mod vfs;
pub use vfs::VFS;

use core::convert::{TryFrom, TryInto};
use thread_group::Credentials;
use vfs::Path;

mod sync;

/// Describe what to do after an IPC request and result return
#[derive(Debug)]
pub enum IpcResult<T> {
    /// Can continue thread execution normally
    Done(T),
    /// the user should wait for his IPC request
    Wait(T, usize),
}

impl<T> IpcResult<T> {
    pub fn expect(self, s: &'static str) -> T {
        match self {
            IpcResult::Done(t) => t,
            IpcResult::Wait(_, _) => panic!(s),
        }
    }
}

pub use process::{
    get_file_content, KernelProcess, Process, ProcessArguments, ProcessOrigin, UserProcess,
};
pub use safe_ffi::{CString, CStringArray};

use scheduler::SCHEDULER;

#[allow(unused)]
use tests::*;

use libc_binding::Errno;
use messaging::MessageTo;

/// SysResult is just made to handle module errors. Return optional return and errno
pub type SysResult<T> = core::result::Result<T, Errno>;

pub trait IntoRawResult {
    fn into_raw_result(self) -> u32;
}

impl IntoRawResult for SysResult<u32> {
    fn into_raw_result(self) -> u32 {
        match self {
            Ok(return_value) => return_value as u32,
            Err(errno) => (-(errno as i32)) as u32,
        }
    }
}

/// MonoTasking or MultiTasking configuration
pub enum TaskMode {
    /// MonoTasking mode
    Mono,
    /// MultiTasking mode, param: frequency
    Multi(f32),
}

use keyboard::keysymb::KeySymb;
use keyboard::{CallbackKeyboard, KEYBOARD_DRIVER};

/// we send a message
pub fn handle_key_press(key_pressed: KeySymb) {
    // in the keyboard interrupt handler, after reading the keysymb,
    // we send a message to the tty which will be handled in the next
    // schedule
    messaging::push_message(MessageTo::Tty { key_pressed })
}

use alloc::sync::Arc;
use core::slice;
use elf_loader::{SegmentType, SymbolTable};
use fallible_collections::FallibleArc;
use kernel_modules::{ForeignAllocMethods, ModConfig, ModResult, ModReturn, SymbolList};

use crate::elf_loader::load_elf;
use crate::memory::mmu::Entry;
use crate::memory::tools::{AllocFlags, Page, Virt};
use crate::memory::KERNEL_VIRTUAL_PAGE_ALLOCATOR;

fn write(s: &str) {
    print!("{}", s);
}

extern "C" {
    fn kmalloc(len: usize) -> *mut u8;
    fn kcalloc(count: usize, size: usize) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn krealloc(addr: *mut u8, new_size: usize) -> *mut u8;
}

fn test() -> SysResult<()> {
    let path = "/turbofish/mod/dummy.mod".try_into()?;
    let content = get_file_content(
        &Path::try_from("/").expect("no root"),
        &Credentials::ROOT,
        path,
    )?;

    // Parse Elf and generate stuff
    let (eip, _symbol_table) = {
        let elf = load_elf(&content)?;
        for h in &elf.program_header_table {
            if h.segment_type == SegmentType::Load {
                let segment = unsafe {
                    KERNEL_VIRTUAL_PAGE_ALLOCATOR.as_mut().unwrap().alloc_on(
                        Virt(h.vaddr as usize).into(),
                        (h.memsz as usize).into(),
                        AllocFlags::KERNEL_MEMORY,
                    )?;
                    slice::from_raw_parts_mut(h.vaddr as usize as *mut u8, h.memsz as usize)
                };
                segment[0..h.filez as usize].copy_from_slice(
                    &content[h.offset as usize..h.offset as usize + h.filez as usize],
                );
                // With BSS (so a NOBITS section), the memsz value exceed the filesz. Setting next bytes as 0

                unsafe {
                    segment[h.filez as usize..h.memsz as usize]
                        .as_mut_ptr()
                        .write_bytes(0, h.memsz as usize - h.filez as usize);
                }
                // Modify the rights on pages by following the ELF specific restrictions
                unsafe {
                    KERNEL_VIRTUAL_PAGE_ALLOCATOR
                        .as_mut()
                        .unwrap()
                        .change_range_page_entry(
                            Page::containing(Virt(h.vaddr as usize)),
                            (h.memsz as usize).into(),
                            &mut |entry: &mut Entry| {
                                *entry |= Entry::from(
                                    Into::<AllocFlags>::into(h.flags) | AllocFlags::KERNEL_MEMORY,
                                )
                            },
                        )?;
                }
            }
        }
        (
            elf.header.entry_point as u32,
            match SymbolTable::try_new(&content).ok() {
                Some(elem) => Some(Arc::try_new(elem)?),
                None => None,
            },
        )
    };

    println!("EIP: {:#X?}", eip);

    let addr: u32 = eip as u32;
    println!("Toto address: {:#X?}", addr);

    let p: fn(SymbolList) -> ModResult = unsafe { core::mem::transmute(addr) };

    let ret = p(SymbolList {
        write,
        alloc_tools: ForeignAllocMethods {
            kmalloc,
            kcalloc,
            kfree,
            krealloc,
        },
        kernel_callback: ModConfig::Dummy,
    });
    println!("ret = {:?}", ret);
    if let Ok(ModReturn::Dummy(dummy_return)) = ret {
        (dummy_return.stop)();
    } else {
        println!("test failed !");
    }
    Ok(())
}

// Create an ASM dummy process based on a simple function
/// Main function of taskMaster Initialisation
pub fn start(filename: &str, argv: &[&str], envp: &[&str]) -> ! {
    // Reassign all cpu exceptions for taskmaster
    unsafe {
        cpu_isr::reassign_cpu_exceptions();
    }
    test().expect("WTF");

    // Initialize Syscall system
    syscall::init();

    // Initialize VFS
    lazy_static::initialize(&VFS);

    // Register the first process
    let path = filename
        .try_into()
        .expect("The path of the init program is not valid");
    let file = get_file_content(&Path::try_from("/").unwrap(), &Credentials::ROOT, path)
        .expect("Cannot syncing");
    SCHEDULER
        .lock()
        .add_user_process(
            1,
            unsafe {
                UserProcess::new(
                    ProcessOrigin::Elf(&file),
                    Some(ProcessArguments::new(
                        argv.try_into().expect("argv creation failed"),
                        envp.try_into().expect("envp creation failed"),
                    )),
                )
            }
            .expect("Unexpected error when parsing ELF file"),
        )
        .expect("Scheduler is bullshit");

    // Register the keyboard callback
    unsafe {
        KEYBOARD_DRIVER
            .as_mut()
            .unwrap()
            .bind(CallbackKeyboard::RequestKeySymb(handle_key_press));
    }

    // Launch the scheduler
    unsafe { scheduler::start(TaskMode::Multi(1000.)) }
}
