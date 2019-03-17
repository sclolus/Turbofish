
#ifndef __EARLY_PANIC_H__
# define __EARLY_PANIC_H__

#include "i386_type.h"

struct base_registers {
	u32 edi, esi, ebp, esp;
	u32 ebx, edx, ecx, eax;
} __attribute__ ((packed));

struct PanicRegisters {
    /*0       |*/ u32 ds;
    /*4       |*/ u32 es;
    /*8       |*/ u32 fs;
    /*12      |*/ u32 gs;
    /*16      |*/ u32 ss;
    /*20      |*/ u32 eip;
    /*24      |*/ u32 cs;
    /*28      |*/ u32 eflags;
    /*32      |*/ struct base_registers regs;
    /*64      |*/ u32 old_ebp;
    /*68      |*/
} __attribute__ ((packed));

void cpu_panic_handler(const char *str, struct PanicRegisters r);

#endif
