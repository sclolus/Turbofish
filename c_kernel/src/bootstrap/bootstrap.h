
#ifndef __BOOTSTRAP_H__
# define __BOOTSTRAP_H__

# include "libft.h"

struct base_registers {
	u32 edi, esi, ebp, esp;
	u32 ebx, edx, ecx, eax;
} __attribute__ ((packed));

int i8086_payload(struct base_registers regs, void *payload, size_t payload_len);

#endif
