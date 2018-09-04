
#ifndef __INT8086_H__
# define __INT8086_H__

# include "i386_type.h"

struct registers {
	u32 edi, esi, ebp, esp;
	u32 ebx, edx, ecx, eax;
} __attribute__ ((packed));

extern void int8086(u8 interupt, struct registers reg);

#endif
