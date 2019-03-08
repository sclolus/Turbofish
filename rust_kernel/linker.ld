/* The bootloader will look at this image and start execution at the symbol
   designated as the entry point. */
ENTRY(_start)

/* Tell where the various sections of the object files will be put in the final
   kernel image. */
SECTIONS
{
	/* Begin putting sections at 1 MiB, a conventional place for kernels to be
	   loaded at by the bootloader. */
	virtual_offset = 0xC0000000;
	. = virtual_offset + 0x00100000;

	kernel_virtual_start = .;
	kernel_physical_start = . - virtual_offset;
	/* First put the multiboot header, as it is required to be put very early
	   early in the image or the bootloader won't recognize the file format.
	   Next we'll put the .text section. */
	.boot BLOCK(4K) : AT(ADDR(.boot) - virtual_offset) ALIGN(4K)
	{
		__start_boot = .;
		*(.multiboot)
		__end_boot = .;
	}

	/* Base code section */
	.text BLOCK(4K) : AT(ADDR(.text) - virtual_offset) ALIGN(4K)
	{
		__start_text = .;
		*(.text)
		*(.text.*)
		__end_text = .;
	}

	/* Read-only data. */
	.rodata BLOCK(4K) : AT(ADDR(.rodata) - virtual_offset) ALIGN(4K)
	{
		__start_rodata = .;
		*(.rodata)
		*(.rodata.*)
		__end_rodata = .;
	}

	/* Read-write data (initialized) */
	.data BLOCK(4K) : AT(ADDR(.data) - virtual_offset) ALIGN(4K)
	{
		__start_data = .;
		*(.data)
		*(.data.*)
		__end_data = .;
	}

	/* For special rust debug section: got mysterious N symbols */
	.debug BLOCK(4k) : AT(ADDR(.debug) - virtual_offset) ALIGN(4K)
	{
		__start_debug = .;
		*(.debug_*)
		*(.comment)
		*(.note.GNU-stack)
		*(.stab)
		*(.stabstr)
		__end_debug = .;
	}

	/* Read-write data (uninitialized) */
	.bss BLOCK(4K) : AT(ADDR(.bss) - virtual_offset) ALIGN(4K)
	{
		__start_bss = .;
		*(.bss)
		*(.bss.*)
		*(COMMON)
		__end_bss = .;
	}
	/* The compiler may produce other sections, by default it will put them in
	   a segment with the same name. Simply add stuff here as needed. */

	kernel_virtual_end = .;
	kernel_physical_end = . - virtual_offset;
}
