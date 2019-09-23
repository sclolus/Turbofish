ENTRY(_start)

/* All the turbo fish section datas MUST be aligned on PAGE_SIZE. Adolf Hitler made the memory allocator.
 * Code injection (for virus, trojan etc...) is more easy with large padding, if we want one day to be
 * more secure, the LD script and the memory virtual allocator MUST be modified. */
SECTIONS
{
	/* Begin putting sections at 4 MiB, a conventional place for turbo fish executable file */
	. = 0x400000;

	/* AX: Base code section */
	.text BLOCK(4K) : ALIGN(4K)
	{
		*(.text)
		*(.text.*)
	}

	/* A: Read-only data. */
	.rodata BLOCK(4K) : ALIGN(4K)
	{
		*(.rodata)
		*(.rodata.*)
	}

	/* A: DWARF-based unwinding for C lang */
	.eh_frame BLOCK(4K) : ALIGN(4K)
	{
		*(.eh_frame)
	}

	/* WA: Relative data. */
	.data.rel.ro BLOCK(4K) : ALIGN(4K)
	{
		*(.data.rel.ro)
		*(.data.rel.ro.*)
	}

	/* WA: Read-write data */
	.data BLOCK(4K) : ALIGN(4K)
	{
		*(.data)
		*(.data.*)
	}

	/* WA: Read-write data */
	.init_array BLOCK(4K) : ALIGN(4K)
	{
		*(.init_array)
	}

	/* WA: Read-write data */
	.fini_array BLOCK(4K) : ALIGN(4K)
	{
		*(.fini_array)
	}

	/* WA: Global Offset Table */
	.got BLOCK(4K) : ALIGN(4K)
	{
		*(.got)
	}

	/* WA: See more about at https://www.segmentationfault.fr/linux/role-plt-got-ld-so/ */
	.got.plt BLOCK(4K) : ALIGN(4K)
	{
		*(.got.plt)
	}

	/* WA: Read-write data must be initialized to 0 by the ELF loader */
	.bss BLOCK(4K) : ALIGN(4K)
	{
		*(.bss)
		*(.bss.*)
	}
}
