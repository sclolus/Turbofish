
#include "i386_type.h"

#define IDT_SIZE 256
#define IDT_ADDRESS 0xC00
#define INTGATE 0x8E00

struct __attribute__ ((packed)) idt_seg {
	u16 offset0_15;
	u16 select;
	u16 type;
	u16 offset16_31;
};

static struct __attribute__ ((packed)) idt_ptr {
	u16 limit;
	u32 base;
} g_idt_ptr;


static int	initialize_idt_seg(u32 nb, u32 fn_addr, u16 select, u16 type)
{
	struct idt_seg *ptr;

	if (nb >= IDT_SIZE)
		return -1;
	ptr = (struct idt_seg *)(IDT_ADDRESS + (nb * sizeof(struct idt_seg)));

	ptr->offset0_15 = fn_addr & 0xFFFF;
	ptr->select = select;
	ptr->type = type;
	ptr->offset16_31 = (fn_addr & 0xFFFF0000) >> 16;
	return 0;
}

extern void	asm_default_interrupt(void);
extern void	asm_divide_by_zero(void);
extern void	asm_page_fault(void);
extern void	asm_default_pic_master_interrupt(void);
extern void	asm_default_pic_slave_interrupt(void);
extern void	asm_clock_handler(void);
extern void	asm_keyboard_handler(void);

void		init_idt(void)
{
	for (int i = 0; i < IDT_SIZE; i++)
		initialize_idt_seg(
				i,
				(u32)&asm_default_interrupt,
				0x8,
				INTGATE);

	for (int i = 32; i <= 39; i++)
		initialize_idt_seg(
				i,
				(u32)&asm_default_pic_master_interrupt,
				0x8,
				INTGATE);

	for (int i = 112; i <= 119; i++)
		initialize_idt_seg(
				i,
				(u32)&asm_default_pic_slave_interrupt,
				0x8,
				INTGATE);

	initialize_idt_seg(0, (u32)&asm_divide_by_zero, 0x8, INTGATE);
	initialize_idt_seg(14, (u32)&asm_page_fault, 0x8, INTGATE);

	initialize_idt_seg(32, (u32)&asm_clock_handler, 0x8, INTGATE);
	initialize_idt_seg(33, (u32)&asm_keyboard_handler, 0x8, INTGATE);

	g_idt_ptr.limit = IDT_SIZE << 3;
	g_idt_ptr.base = IDT_ADDRESS;

	asm("lidt (g_idt_ptr)");
}
