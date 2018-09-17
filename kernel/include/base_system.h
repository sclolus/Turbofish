
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
void		init_pic(void);

int		init_paging(u32 available_memory);
extern void	asm_paging_enable(void);
extern void	asm_paging_disable(void);
extern void	asm_paging_set_page_directory_address(
			ptr_32 *page_directory_address);

struct base_registers {
	u32 edi, esi, ebp, esp;
	u32 ebx, edx, ecx, eax;
} __attribute__ ((packed));

/*
 * When a CPU IRQ interrupt occurred, registers are pushed in this order,
 * the following C structure match with that
 *
 * push ebp
 * mov ebp, esp
 * definition of %macro PUSH_ALL_REGISTERS 0
 * pushad                ; EAX, ECX, EDX, EBX, and ESP, EBP, ESI, EDI
 * push dword [ebp + 16] ; eflags
 * push dword [ebp + 12] ; cs
 * push dword [ebp + 8]  ; eip
 * push ss
 * push es
 * push ds
 * %endmacro
 */

struct extended_registers {
	u32 ds, es, ss;
	u32 eip;
	u32 cs;
	u32 eflags;
	u32 edi, esi, new_ebp, esp;
	u32 ebx, edx, ecx, eax;
	u32 old_ebp;
} __attribute__ ((packed));

void		panic(const char *s, struct extended_registers reg);

void		bios_shutdown_computer(void);
void		bios_wait(u32 sec);

extern void	int8086(u8 interupt, struct base_registers reg);

#endif
