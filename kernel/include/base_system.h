
#ifndef __BASE_SYSTEM_H__
# define __BASE_SYSTEM_H__

# include "i386_type.h"

extern void		init_GDT(ptr_32 *LFB);
void			bios_shutdown_computer(void);
void			bios_wait(u32 sec);

struct registers {
	u32 edi, esi, ebp, esp;
	u32 ebx, edx, ecx, eax;
} __attribute__ ((packed));

extern void		int8086(u8 interupt, struct registers reg);

#endif
