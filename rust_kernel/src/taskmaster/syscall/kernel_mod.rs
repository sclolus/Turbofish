//! sys_rmmod() && sys_insmod

use super::process::get_file_content;
use super::scheduler::{Scheduler, SCHEDULER};
use super::thread_group::Credentials;
use super::vfs::Path;
use super::SysResult;

use alloc::boxed::Box;
use alloc::vec::Vec;
use elf_loader::{SegmentType, SymbolTable};
use fallible_collections::boxed::FallibleBox;
use kernel_modules::{
    ForeignAllocMethods, ModConfig, ModResult, ModReturn, ModStatus, Status, SymbolList,
};
use libc_binding::{c_char, Errno};

use core::convert::{TryFrom, TryInto};
use core::slice;

use crate::elf_loader::load_elf;
use crate::memory::mmu::Entry;
use crate::memory::tools::{AllocFlags, NbrPages, Page, Virt};
use crate::memory::KERNEL_VIRTUAL_PAGE_ALLOCATOR;

/// Insert a kernel module
pub fn sys_insmod(modname: *const c_char) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let safe_modname = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            v.make_checked_str(modname)?
        };
        scheduler.insert_module(safe_modname)
    })
}

/// Remove a kernel module
pub fn sys_rmmod(modname: *const c_char) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let safe_modname = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            v.make_checked_str(modname)?
        };
        scheduler.remove_module(safe_modname)
    })
}

/// Main structure
pub struct KernelModules {
    /// this structure may be shared between modules to check dependencies
    status: ModStatus,
    dummy: Option<Module>,
}

/// Main implementation
impl KernelModules {
    pub fn new() -> Self {
        Self {
            status: ModStatus::default(),
            dummy: None,
        }
    }
}

/// Stored structure of a given module
struct Module {
    start_point: u32,
    symbol_table: Box<SymbolTable>,
    mod_return: ModReturn,
    alloc_table: Vec<AllocEntry>,
}

struct AllocEntry {
    page_index: Page<Virt>,
    nbr_pages: NbrPages,
}

impl AllocEntry {
    fn free(&self) -> SysResult<()> {
        unsafe {
            KERNEL_VIRTUAL_PAGE_ALLOCATOR
                .as_mut()
                .unwrap()
                .dealloc_on(self.page_index, self.nbr_pages)
                .map_err(|e| e.into())
        }
    }
}

impl Scheduler {
    /// Try to insert a Kernel Module
    fn insert_module(&mut self, modname: &str) -> SysResult<u32> {
        match modname {
            "dummy" => {
                // Check if dummy module is already loaded
                if Status::Active == self.kernel_modules.status.dummy {
                    log::warn!("Dummy Module already active");
                    return Ok(0);
                }
                // Generate content from disk
                let content = get_module_raw_content("/turbofish/mod/dummy.mod")?;
                // Try to parse ELF
                let (eip, symbol_table, alloc_table) = load_module(&content)?;

                let symbol_table = match symbol_table {
                    Some(s) => s,
                    None => {
                        log::error!("No Symtab for Dummy Module");
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
                    kernel_callback: ModConfig::Dummy,
                })
                .map_err(|_e| {
                    /* Free memory */
                    Errno::EINVAL
                })?;

                self.kernel_modules.status.dummy = Status::Active;
                self.kernel_modules.dummy = Some(Module {
                    start_point,
                    symbol_table,
                    mod_return,
                    alloc_table,
                });
                log::info!("Inserting Dummy Module");
            }
            _ => unimplemented!(),
        }
        Ok(0)
    }

    /// Try to remove a kernel module
    fn remove_module(&mut self, modname: &str) -> SysResult<u32> {
        match modname {
            "dummy" => {
                // Check if dummy module is already unloaded
                if Status::Inactive == self.kernel_modules.status.dummy {
                    log::warn!("Dummy Module already inactive");
                    return Ok(0);
                }
                // Invoque Stop fn
                if let ModReturn::Dummy(dummy_return) =
                    self.kernel_modules.dummy.as_ref().expect("WOOT").mod_return
                {
                    (dummy_return.stop)();
                } else {
                    panic!("WTF");
                }
                for elem in self
                    .kernel_modules
                    .dummy
                    .as_mut()
                    .expect("WTF")
                    .alloc_table
                    .iter()
                {
                    elem.free().expect("WOOT 2");
                }
                self.kernel_modules.status.dummy = Status::Inactive;
                self.kernel_modules.dummy = None;
                log::info!("Removing Dummy Module");
            }
            _ => unimplemented!(),
        }
        Ok(0)
    }
}

/// Common Write method for modules
fn write(s: &str) {
    print!("{}", s);
}

/// Common allocator methods for modules
extern "C" {
    fn kmalloc(len: usize) -> *mut u8;
    fn kcalloc(count: usize, size: usize) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn krealloc(addr: *mut u8, new_size: usize) -> *mut u8;
}

/// Load a module from ELF
fn load_module(content: &[u8]) -> SysResult<(u32, Option<Box<SymbolTable>>, Vec<AllocEntry>)> {
    let mut alloc_table: Vec<AllocEntry> = Vec::new();
    // Parse Elf and generate stuff
    let elf = load_elf(content)?;
    for h in &elf.program_header_table {
        if h.segment_type == SegmentType::Load {
            let segment = unsafe {
                let page_index: Page<Virt> = Virt(h.vaddr as usize).into();
                let nbr_pages: NbrPages = (h.memsz as usize).into();

                KERNEL_VIRTUAL_PAGE_ALLOCATOR.as_mut().unwrap().alloc_on(
                    page_index,
                    nbr_pages,
                    AllocFlags::KERNEL_MEMORY,
                )?;
                // TODO: Make fallible
                alloc_table.push(AllocEntry {
                    page_index,
                    nbr_pages,
                });
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
