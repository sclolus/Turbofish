
#ifndef __KERNEL_IO_H__
# define __KERNEL_IO_H__

# include "i386_type.h"

enum term_mode {
	boot = 0,
	kernel,
	user
};

struct kernel_io_ctx {
	enum term_mode term_mode;
} g_kernel_io_ctx;

s32	write(s32 fd, const void *buf, u32 size);

#endif
