/* The bootloader will look at this image and start execution at the symbol
   designated as the entry point. */
ENTRY(_start)

/* Tell where the various sections of the object files will be put in the final
   kernel image. */
SECTIONS
{
	/* Begin putting sections at 1 MiB, a conventional place for kernels to be
	   loaded at by the bootloader. */

	. = 0x100000;
	kernel_physical_start = .;

	/* First put the multiboot header, as it is required to be put very early
	   early in the image or the bootloader won't recognize the file format.
	   Next we'll put the .text section. */

	.boot BLOCK(4K) : ALIGN(4K)
	{
		*(.multiboot)
	}

	.init BLOCK(4K) : ALIGN(4K)
	{
		*(.init)
		*(.init.*)
		objs_init/get_device_mem_map.o
		objs_init/vga_text.o
		objs_init/8086_payload.o
		objs_init/payload_get_mem_map.o
		objs_init/early_gdt.o
		objs_init/init_paging.o
		/* I dont know why but the lib must be at the end of section, if else symbols will be in high mem area */
		libft/alt_libft.a
	}

	virtual_offset = 0xc0000000;
	. += virtual_offset;

	/* Base code section */
	.text BLOCK(4K) : AT(ADDR(.text) - virtual_offset) ALIGN(4K)
	{
		*(.text)
		*(.text.*)
	}

	/* Read-only data. */
	.rodata BLOCK(4K) : AT(ADDR(.rodata) - virtual_offset) ALIGN(4K)
	{
		*(.rodata)
		*(.rodata.*)
	}

	/* Read-write data (initialized) */
	.data BLOCK(4K) : AT(ADDR(.data) - virtual_offset) ALIGN(4K)
	{
		*(.data)
		*(.data.*)
	}

	/* For special rust debug section: got mysterious N symbols */
	.debug BLOCK(4k) : AT(ADDR(.debug) - virtual_offset) ALIGN(4K)
	{
		*(.debug_*)
		*(.comment)
		*(.note.GNU-stack)
		*(.stab)
		*(.stabstr)
	}

	/* Read-write data (uninitialized) */
	.bss BLOCK(4K) : AT(ADDR(.bss) - virtual_offset) ALIGN(4K)
	{
		*(.bss)
		*(.bss.*)
		*(COMMON)
	}
	/* The compiler may produce other sections, by default it will put them in
	   a segment with the same name. Simply add stuff here as needed. */

	kernel_virtual_end = .;
	kernel_physical_end = . - virtual_offset;
}
