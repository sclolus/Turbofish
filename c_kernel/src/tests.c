
#include "i386_type.h"
#include "libft.h"

#include "watchdog.h"

struct base_registers {
	u32 edi, esi, ebp, esp;
	u32 ebx, edx, ecx, eax;
} __attribute__ ((packed));

extern u32 _real_mode_op(struct base_registers reg, u16 bios_int);

int alt_printk(const char *restrict format, ...);

#define VESA_GLOBAL_INFO_PTR 0x2000

extern u8 __start_text;
extern u8 __end_text;
extern u8 __start_rodata;
extern u8 __end_rodata;

u32 bench(void *symbol_start, void *symbol_end)
{
	char *start = (char *)symbol_start;
	char *end = (char *)symbol_end;
	u32 result = 0;
	while (start != end) {
		result += *start;
		start++;
	}
	return result;
}

void tests(void)
{
	struct base_registers reg;

	alt_printk("guards setup\n");

	dog_guard(idt_bios);
	dog_guard(gdt);
	dog_guard(idt);

	u32 a = bench(&__start_text, &__end_text);
	u32 c = bench(&__start_rodata, &__end_rodata);

	// get global VBE info
	for (int i = 0; i < 256; i++) {
		ft_memset(&reg, 0, sizeof(struct base_registers));
		reg.eax = 0x4F00;
		reg.edi = VESA_GLOBAL_INFO_PTR;
		_real_mode_op(reg, 0x10);
	}

	dog_bark(idt_bios);
	dog_bark(gdt);
	dog_bark(idt);

	for (int i = 0; i < 256; i++) {
	// get selected mode info include LFB address location
		ft_memset(&reg, 0, sizeof(struct base_registers));
		reg.eax = 0x4F01;
		reg.ecx = 0x118 | (1 << 14);	// CX 1 << 14 => LFB
		reg.edi = VESA_GLOBAL_INFO_PTR;
		_real_mode_op(reg, 0x10);
	}

	dog_bark(idt_bios);
	dog_bark(gdt);
	dog_bark(idt);

	for (int i = 0; i < 256; i++) {
		ft_memset(&reg, 0, sizeof(struct base_registers));
		reg.eax = 0x4F02;
		reg.ebx = 0x118 | (1 << 14);	// CX 1 << 14 => LFB
		reg.edi = VESA_GLOBAL_INFO_PTR;
		_real_mode_op(reg, 0x10);
	}

	// get back to VGA mode
	ft_memset(&reg, 0, sizeof(struct base_registers));
	reg.eax = 0x0;
	reg.ebx = 0x13;
	reg.edi = VESA_GLOBAL_INFO_PTR;
	_real_mode_op(reg, 0x10);

	u32 b = bench(&__start_text, &__end_text);
	u32 d = bench(&__start_rodata, &__end_rodata);

	alt_printk("diff: %u bench %u:%u %u:%u\n", &__end_text - &__start_text, a, b, c, d);
	while (1);

	dog_bark(idt_bios);
	dog_bark(gdt);
	dog_bark(idt);
}
