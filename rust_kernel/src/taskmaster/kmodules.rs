//! This file contains all the stuff needed by Kernel Modules
use super::message::push_message;
use super::process::get_file_content;
use super::scheduler::Scheduler;
use super::thread_group::Credentials;
use super::vfs::{Path, VFS};
use super::{IpcResult, SysResult};

use alloc::boxed::Box;
use alloc::vec::Vec;
use elf_loader::{SegmentType, SymbolTable};
use fallible_collections::boxed::FallibleBox;
use irq::Irq;
use kernel_modules::{
    ForeignAllocMethods, KernelEvent, KernelSymbolList, KeyboardConfig, ModConfig, ModResult,
    ModReturn, ModSpecificReturn, RTCConfig, SymbolList,
};
use libc_binding::{Errno, FileType, OpenFlags};
use log::Record;
use time::Date;

use core::convert::{TryFrom, TryInto};
use core::slice;
use core::sync::atomic::AtomicU32;

use crate::drivers::PIC_8259;
use crate::elf_loader::load_elf;
use crate::memory::mmu::Entry;
use crate::memory::tools::{AllocFlags, NbrPages, Page, Virt};
use crate::memory::HIGH_KERNEL_MEMORY;

/// Main structure
pub struct KernelModules {
    dummy: Option<Module>,
    rtc: Option<Module>,
    keyboard: Option<Module>,
    syslog: Option<Module>,
    pub second_cycle: Vec<fn()>,
}

#[allow(dead_code)]
/// Stored structure of a given module
struct Module {
    start_point: u32,
    symbol_table: Box<SymbolTable>,
    mod_return: ModReturn,
    alloc_table: AllocTable,
}

/// Main implementation
impl KernelModules {
    pub fn new() -> Self {
        Self {
            dummy: None,
            rtc: None,
            keyboard: None,
            syslog: None,
            second_cycle: Vec::new(),
        }
    }
}

impl Scheduler {
    /// Try to insert a Kernel Module
    pub fn insert_module(&mut self, modname: &str) -> SysResult<u32> {
        let (module_opt, module_pathname, mod_config) = match modname {
            "dummy" => (
                &mut self.kernel_modules.dummy,
                "/turbofish/mod/dummy.mod",
                ModConfig::Dummy,
            ),
            "rtc" => (
                &mut self.kernel_modules.rtc,
                "/turbofish/mod/rtc.mod",
                ModConfig::RTC(RTCConfig {
                    enable_irq,
                    disable_irq,
                    // May be set as volatile...
                    current_unix_time: unsafe { &mut CURRENT_UNIX_TIME },
                }),
            ),
            "keyboard" => (
                &mut self.kernel_modules.keyboard,
                "/turbofish/mod/key.mod",
                ModConfig::Keyboard(KeyboardConfig {
                    enable_irq,
                    disable_irq,
                    callback: push_message,
                }),
            ),
            "syslog" => (
                &mut self.kernel_modules.syslog,
                "/turbofish/mod/syslog.mod",
                ModConfig::Syslog,
            ),
            _ => {
                log::warn!("Unknown module name");
                return Ok(0);
            }
        };

        if let Some(_) = module_opt {
            log::warn!("Module already active");
            return Ok(0);
        }

        // Generate content from disk
        let content = get_module_raw_content(module_pathname)?;
        // Try to parse ELF
        let (eip, symbol_table, alloc_table) = load_module(&content)?;

        let symbol_table = match symbol_table {
            Some(s) => s,
            None => {
                log::error!("No Symtab for that Module");
                return Err(Errno::EINVAL);
            }
        };

        // Launch the module with his particulary context
        let start_point: u32 = eip as u32;
        let p: fn(SymbolList) -> ModResult = unsafe { core::mem::transmute(start_point) };

        let mod_return = p(SymbolList {
            write,
            alloc_tools: ForeignAllocMethods {
                kmalloc,
                kcalloc,
                kfree,
                krealloc,
            },
            kernel_callback: mod_config,
            kernel_symbol_list: KernelSymbolList::new(),
        })
        .map_err(|_e| Errno::EINVAL)?;

        if let Some(configurable_callbacks) = &mod_return.configurable_callbacks_opt {
            // Ensure we have suffisant memory before binding something
            let mut second_cycle_chunk_reserved = 0;
            for elem in configurable_callbacks.iter() {
                match elem.when {
                    KernelEvent::Second => second_cycle_chunk_reserved += 1,
                    _ => {}
                }
            }
            self.kernel_modules
                .second_cycle
                .try_reserve(second_cycle_chunk_reserved)?;

            // Bind callbacks
            for elem in configurable_callbacks.iter() {
                match elem.when {
                    KernelEvent::Log => {
                        // We assume that a function bindable to Log event has fn(&Record) prototype.
                        // Yes, it is really really unsafe... But Louis is asking for that
                        // LOGGER is on a direct binding. Not passing through Scheduler
                        let p: fn(&Record) = unsafe { core::mem::transmute(elem.what) };
                        unsafe {
                            // It is a shame that only one module can be binded to the log !
                            terminal::log::LOGGER.bind(p);
                        }
                    }
                    KernelEvent::Second => {
                        // We assume that a function bindable to Log event has fn() prototype.
                        let p: fn() = unsafe { core::mem::transmute(elem.what) };
                        self.kernel_modules.second_cycle.push(p);
                    }
                }
            }
        }

        *module_opt = Some(Module {
            start_point,
            symbol_table,
            mod_return,
            alloc_table,
        });
        Ok(0)
    }

    /// Try to remove a kernel module
    pub fn remove_module(&mut self, modname: &str) -> SysResult<u32> {
        let module_opt = match modname {
            "dummy" => &mut self.kernel_modules.dummy,
            "rtc" => &mut self.kernel_modules.rtc,
            "keyboard" => &mut self.kernel_modules.keyboard,
            "syslog" => &mut self.kernel_modules.syslog,
            _ => {
                log::warn!("Unknown module name");
                return Ok(0);
            }
        };
        match module_opt {
            None => {
                log::warn!("Module already inactive");
                return Ok(0);
            }
            Some(module) => {
                // Disable callbacks
                if let Some(configurable_callbacks) = &module.mod_return.configurable_callbacks_opt
                {
                    for elem in configurable_callbacks.iter() {
                        match elem.when {
                            KernelEvent::Log => unsafe {
                                terminal::log::LOGGER.unbind();
                            },
                            KernelEvent::Second => {
                                let p: fn() = unsafe { core::mem::transmute(elem.what) };
                                let _r = self
                                    .kernel_modules
                                    .second_cycle
                                    .drain_filter(|elem| *elem == p)
                                    .collect::<Vec<_>>();
                            }
                        }
                    }
                }
                // Halt the module
                (module.mod_return.stop)();
            }
        }
        *module_opt = None;
        Ok(0)
    }

    /// Keyboard driver method specific
    pub fn reboot_computer(&self) {
        if let Some(keyboard) = &self.kernel_modules.keyboard {
            if let ModSpecificReturn::Keyboard(keyboard_return) = &keyboard.mod_return.spec {
                (keyboard_return.reboot_computer)();
            } else {
                panic!("Unexpected error");
            }
        } else {
            log::error!("ps2_controler/Keyboard handler not loaded");
        }
    }

    /// RTC driver method specific
    pub fn read_date(&self) -> Date {
        if let Some(rtc) = &self.kernel_modules.rtc {
            if let ModSpecificReturn::RTC(rtc_return) = &rtc.mod_return.spec {
                (rtc_return.read_date)()
            } else {
                panic!("Unexpected error");
            }
        } else {
            Date::default()
        }
    }
}

/// RTC driver specific globale
pub static mut CURRENT_UNIX_TIME: AtomicU32 = AtomicU32::new(0);

/// Set IDT ENTRY fn: Usable by modules
fn enable_irq(idt_gate: Irq, func: unsafe extern "C" fn()) {
    unsafe {
        PIC_8259.lock().enable_irq(idt_gate, Some(func));
    }
}

/// Unset IDT ENTRY fn: Usable by modules
fn disable_irq(idt_gate: Irq) {
    unsafe {
        PIC_8259.lock().disable_irq(idt_gate);
    }
}

/// Common Write method for modules
fn write(s: &str) {
    log::info!("{}", s);
}

/// Just used for a symbol list test
#[no_mangle]
#[link_section = ".kernel_exported_functions"]
pub fn symbol_list_test() {
    log::info!("symbol_list_test function sucessfully called by a module !");
}

#[no_mangle]
#[link_section = ".kernel_exported_functions"]
pub fn add_syslog_entry(entry: &str) -> Result<(), Errno> {
    let cwd = Path::try_from("/")?;
    let path = Path::try_from("/var/syslog")?;
    let mode = FileType::from_bits(0o600).expect("Cannot set FileType");
    let flags = OpenFlags::O_WRONLY | OpenFlags::O_CREAT | OpenFlags::O_APPEND;
    let creds = &Credentials::ROOT;
    VFS.force_unlock(); /* just in case of. This mutex could become very problematic */
    let file_operator = match VFS.lock().open(&cwd, creds, path, flags, mode)? {
        IpcResult::Done(file_operator) => file_operator,
        IpcResult::Wait(file_operator, _) => file_operator,
    };
    let mut m = file_operator.lock();
    m.write(unsafe { core::slice::from_raw_parts(entry as *const _ as *const u8, entry.len()) })?;
    Ok(())
}

/// Common allocator methods for modules
extern "C" {
    fn kmalloc(len: usize) -> *mut u8;
    fn kcalloc(count: usize, size: usize) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn krealloc(addr: *mut u8, new_size: usize) -> *mut u8;
}

struct AllocTable(Vec<AllocEntry>);

struct AllocEntry {
    page_index: Page<Virt>,
    nbr_pages: NbrPages,
}

impl AllocEntry {
    fn new(page_index: Page<Virt>, nbr_pages: NbrPages) -> Self {
        Self {
            page_index,
            nbr_pages,
        }
    }
}

impl Drop for AllocEntry {
    fn drop(&mut self) {
        unsafe {
            HIGH_KERNEL_MEMORY
                .as_mut()
                .unwrap()
                .dealloc_on(self.page_index, self.nbr_pages)
                .expect("Unexpected memory error");
        }
    }
}

/// Load a module from ELF
fn load_module(content: &[u8]) -> SysResult<(u32, Option<Box<SymbolTable>>, AllocTable)> {
    let mut alloc_table: AllocTable = AllocTable(Vec::new());
    // Parse Elf and generate stuff
    let elf = load_elf(content)?;
    for h in &elf.program_header_table {
        if h.segment_type == SegmentType::Load {
            let segment = unsafe {
                let page_index: Page<Virt> = Virt(h.vaddr as usize).into();
                let nbr_pages: NbrPages = (h.memsz as usize).into();
                alloc_table.0.try_reserve(1)?;
                HIGH_KERNEL_MEMORY.as_mut().unwrap().alloc_on(
                    page_index,
                    nbr_pages,
                    AllocFlags::KERNEL_MEMORY,
                )?;
                alloc_table.0.push(AllocEntry::new(page_index, nbr_pages));
                slice::from_raw_parts_mut(h.vaddr as usize as *mut u8, h.memsz as usize)
            };
            segment[0..h.filez as usize]
                .copy_from_slice(&content[h.offset as usize..h.offset as usize + h.filez as usize]);
            unsafe {
                // With BSS (so a NOBITS section), the memsz value exceed the filesz. Setting next bytes as 0
                segment[h.filez as usize..h.memsz as usize]
                    .as_mut_ptr()
                    .write_bytes(0, h.memsz as usize - h.filez as usize);
                // Modify the rights on pages by following the ELF specific restrictions
                HIGH_KERNEL_MEMORY
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
    Ok((
        elf.header.entry_point as u32,
        match SymbolTable::try_new(content).ok() {
            Some(elem) => Some(Box::try_new(elem)?),
            None => None,
        },
        alloc_table,
    ))
}

/// Get Data of a module
fn get_module_raw_content(mod_pathname: &str) -> SysResult<Vec<u8>> {
    let path = mod_pathname.try_into()?;
    get_file_content(
        &Path::try_from("/").expect("no root"),
        &Credentials::ROOT,
        path,
    )
}
