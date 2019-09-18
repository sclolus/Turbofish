//! sys_rmmod() && sys_insmod

use super::scheduler::SCHEDULER;
use super::SysResult;

use libc_binding::c_char;

/// Insert a kernel module
pub fn sys_insmod(modname: *const c_char) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let _safe_modname = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            v.make_checked_str(modname)?
        };
        test().expect("WOOT");
        Ok(0)
    })
}

/// Remove a kernel module
pub fn sys_rmmod(modname: *const c_char) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let _safe_modname = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            v.make_checked_str(modname)?
        };
        Ok(0)
    })
}

use super::process::get_file_content;
use super::thread_group::Credentials;
use super::vfs::Path;

use alloc::sync::Arc;
use elf_loader::{SegmentType, SymbolTable};
use fallible_collections::FallibleArc;
use kernel_modules::{ForeignAllocMethods, ModConfig, ModResult, ModReturn, SymbolList};

use core::convert::{TryFrom, TryInto};
use core::slice;

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
