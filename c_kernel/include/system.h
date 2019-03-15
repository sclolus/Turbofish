
#ifndef __SYSTEM_H__
# define __SYSTEM_H__

# include "i386_type.h"

extern void	init_gdt(u32 linear_frame_buffer);
void		init_idt(void);
void		init_pic(void);
int		init_paging(u32 available_memory, u32 *vesa_framebuffer);

struct base_registers {
	u32 edi, esi, ebp, esp;
	u32 ebx, edx, ecx, eax;
} __attribute__ ((packed));

/*
 * 8254_pit.asm Time system
 */
struct timeval {
	u32 sec;
	u32 usec;
};

void		asm_pit_init(u32 frequency);

int		clock_gettime(struct timeval *tv);

/*
 * When a CPU IRQ interrupt occurred, registers are pushed in this order,
 * the following C structure match with that
 *
 * push ebp
 * mov ebp, esp
 * definition of %macro PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET 0
 * pushad                ; EAX, ECX, EDX, EBX, and ESP, EBP, ESI, EDI
 * push dword [ebp + 16] ; eflags
 * push dword [ebp + 12] ; cs
 * push dword [ebp + 8]  ; eip
 * push ss
 * push es
 * push ds
 * %endmacro
 *
 * There is the same macro for no_err_code IRQ without offset + 4
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

extern void	_int8086(struct base_registers *reg, u16 interupt);

#endif
