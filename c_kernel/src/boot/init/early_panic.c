
#include "early_panic.h"

#include "libft.h"

void cpu_panic_handler(const char *str, struct PanicRegisters r)
{
	printk("{red}Panic -> %s\n", str);
	printk("CS: %#0.2hx DS: %#0.2hx ES: %#0.2hx FS: %#0.2hx GS: %#0.2hx SS: %#0.2hx\n",
			r.cs, r.ds, r.es, r.fs, r.gs, r.ss);
	printk("EIP: %.8p EFLAGS: %.8p OLD_EBP: %.8p\n",
			r.eip, r.eflags, r.old_ebp);
	printk("EAX: %.8p EBX: %.8p ECX: %.8p EDX: %.8p\n",
			r.regs.eax, r.regs.ebx, r.regs.ecx, r.regs.edx);
	printk("ESI: %.8p EDI: %.8p EBP: %.8p ESP: %.8p\n",
			r.regs.esi, r.regs.edi, r.regs.ebp, r.regs.esp);
	while (1) {}
}
