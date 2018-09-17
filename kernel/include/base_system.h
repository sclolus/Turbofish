
#ifndef __BASE_SYSTEM_H__
# define __BASE_SYSTEM_H__

# include "i386_type.h"

extern void	init_gdt(ptr_32 *LFB);
void		init_idt(void);
int		initialize_idt_seg(
			u32 nb,
			u32 fn_addr,
			u16 select,
			u16 type);
void			init_pic(void);

int		init_paging(u32 available_memory);
extern void	asm_paging_enable(void);
extern void	asm_paging_disable(void);
extern void	asm_paging_set_page_directory_address(
			ptr_32 *page_directory_address);

struct registers {
	u32 edi, esi, ebp, esp;
	u32 ebx, edx, ecx, eax;
} __attribute__ ((packed));

void		panic(
			const char *s,
			u32 ds,
			u32 es,
			u32 ss,
			u32 eip,
			u32 cs,
			u32 eflags,
			struct registers reg,
			u32 ebp);

void		bios_shutdown_computer(void);
void		bios_wait(u32 sec);

extern void	int8086(u8 interupt, struct registers reg);

#endif
