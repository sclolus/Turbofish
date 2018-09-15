
#ifndef __KERNEL_IO_H__
# define __KERNEL_IO_H__

# include "i386_type.h"

enum term_mode {
	boot = 0,
	kernel,
	user
};

#define BOOT_STORE_BASE_ADDR	0x8000

struct kernel_io_ctx {
	enum term_mode term_mode;
	u8 *boot_char_ptr;
} g_kernel_io_ctx;

void	init_kernel_io_ctx(void);
void	fflush_boot_storage(void);
void	set_kernel_io_mode(void);

s32	write(s32 fd, const void *buf, u32 size);

#endif
