use super::{Driver, FileOperation, IpcResult, SysResult};

use alloc::sync::Arc;

use fallible_collections::FallibleArc;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::Errno;

#[derive(Debug, Clone)]
pub struct VmstatDriver;

unsafe impl Send for VmstatDriver {}

impl Driver for VmstatDriver {
    fn open(&mut self, _flags: OpenFlags) -> SysResult<IpcResult<Arc<Mutex<dyn FileOperation>>>> {
        let res = Arc::try_new(Mutex::new(VmstatOperations { offset: 0 }))?;
        Ok(IpcResult::Done(res))
    }
}

#[derive(Debug, Default)]
pub struct VmstatOperations {
    // offset: u64,
    offset: usize,
}

impl FileOperation for VmstatOperations {
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        if buf.len() > u32::max_value() as usize {
            return Err(Errno::EOVERFLOW);
        }

        //TODO: Unfailible context.
        //TODO: This is dummy.
        let vmstat_string = format!(
            "nr_free_pages 0
nr_zone_inactive_anon 0
nr_zone_active_anon 0
nr_zone_inactive_file 0
nr_zone_active_file 0
nr_zone_unevictable 0
nr_zone_write_pending 0
nr_mlock 0
nr_page_table_pages 0
nr_kernel_stack 0
nr_bounce 0
nr_zspages 0
nr_free_cma 0
numa_hit 0
numa_miss 0
numa_foreign 0
numa_interleave 0
numa_local 0
numa_other 0
nr_inactive_anon 0
nr_active_anon 0
nr_inactive_file 0
nr_active_file 0
nr_unevictable 0
nr_slab_reclaimable 0
nr_slab_unreclaimable 0
nr_isolated_anon 0
nr_isolated_file 0
workingset_refault 0
workingset_activate 0
workingset_nodereclaim 0
nr_anon_pages 0
nr_mapped 0
nr_file_pages 0
nr_dirty 0
nr_writeback 0
nr_writeback_temp 0
nr_shmem 0
nr_shmem_hugepages 0
nr_shmem_pmdmapped 0
nr_anon_transparent_hugepages 0
nr_unstable 0
nr_vmscan_write 0
nr_vmscan_immediate_reclaim 0
nr_dirtied 0
nr_written 0
nr_dirty_threshold 0
nr_dirty_background_threshold 0
pgpgin 0
pgpgout 0
pswpin 0
pswpout 0
pgalloc_dma 0
pgalloc_dma32 0
pgalloc_normal 0
pgalloc_movable 0
allocstall_dma 0
allocstall_dma32 0
allocstall_normal 0
allocstall_movable 0
pgskip_dma 0
pgskip_dma32 0
pgskip_normal 0
pgskip_movable 0
pgfree 0
pgactivate 0
pgdeactivate 0
pglazyfree 0
pgfault 0
pgmajfault 0
pglazyfreed 0
pgrefill 0
pgsteal_kswapd 0
pgsteal_direct 0
pgscan_kswapd 0
pgscan_direct 0
pgscan_direct_throttle 0
zone_reclaim_failed 0
pginodesteal 0
slabs_scanned 0
kswapd_inodesteal 0
kswapd_low_wmark_hit_quickly 0
kswapd_high_wmark_hit_quickly 0
pageoutrun 0
pgrotated 0
drop_pagecache 0
drop_slab 0
oom_kill 0
numa_pte_updates 0
numa_huge_pte_updates 0
numa_hint_faults 0
numa_hint_faults_local 0
numa_pages_migrated 0
pgmigrate_success 0
pgmigrate_fail 0
compact_migrate_scanned 0
compact_free_scanned 0
compact_isolated 0
compact_stall 0
compact_fail 0
compact_success 0
compact_daemon_wake 0
compact_daemon_migrate_scanned 0
compact_daemon_free_scanned 0
htlb_buddy_alloc_success 0
htlb_buddy_alloc_fail 0
unevictable_pgs_culled 0
unevictable_pgs_scanned 0
unevictable_pgs_rescued 0
unevictable_pgs_mlocked 0
unevictable_pgs_munlocked 0
unevictable_pgs_cleared 0
unevictable_pgs_stranded 0
thp_fault_alloc 0
thp_fault_fallback 0
thp_collapse_alloc 0
thp_collapse_alloc_failed 0
thp_file_alloc 0
thp_file_mapped 0
thp_split_page 0
thp_split_page_failed 0
thp_deferred_split_page 0
thp_split_pmd 0
thp_split_pud 0
thp_zero_page_alloc 0
thp_zero_page_alloc_failed 0
thp_swpout 0
thp_swpout_fallback 0
balloon_inflate 0
balloon_deflate 0
balloon_migrate 0
swap_ra 0
swap_ra_hit 0
"
        );

        if self.offset >= vmstat_string.len() {
            return Ok(IpcResult::Done(0));
        }

        let version = &vmstat_string[self.offset as usize..];

        let mut bytes = version.bytes();

        let mut ret = 0;
        for (index, to_fill) in buf.iter_mut().enumerate() {
            match bytes.next() {
                Some(byte) => *to_fill = byte,
                None => {
                    ret = index + 1;
                    break;
                }
            }
        }
        self.offset += ret;
        Ok(IpcResult::Done(ret as u32))
    }
}
