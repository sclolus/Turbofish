#[cfg(feature = "test-failed")]
pub mod failed_test;
#[cfg(feature = "test-succeed")]
pub mod succeed_test;

pub mod helpers;
#[cfg(feature = "test-sodo-allocator")]
pub mod sodo_allocator;

#[cfg(feature = "test-vmalloc")]
#[path = "tests"]
mod reexport_test_vmalloc {
    pub mod standard_sodomizer;
    pub mod vmalloc;
}

#[cfg(feature = "test-kmalloc")]
#[path = "tests"]
mod reexport_test_kmalloc {
    pub mod kmalloc;
    pub mod standard_sodomizer;
}

#[cfg(feature = "native-test-hard-drive-read-pio")]
pub mod hard_drive_read_pio;

#[cfg(feature = "native-test-hard-drive-write-pio")]
pub mod hard_drive_write_pio;

#[cfg(feature = "native-test-hard-drive-read-bios")]
pub mod hard_drive_read_bios;

#[cfg(feature = "native-test-hard-drive-write-bios")]
pub mod hard_drive_write_bios;
