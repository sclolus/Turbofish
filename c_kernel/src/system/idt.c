
#include "i386_type.h"

/*
 * IDT region: 0x1000 -> 0x1800 (IDT_SIZE * SIZE(IDT_SEGMENT) = 256 * 8 = 2000 = 0x800)
 */
#define IDT_SIZE 256
#define IDT_ADDRESS 0x1000
#define INTGATE 0x8E00

struct __attribute__ ((packed)) idt_seg {
	u16 offset0_15;
	u16 select;
	u16 type;
	u16 offset16_31;
};

struct __attribute__ ((packed)) idt_ptr {
	u16 limit;
	u32 base;
} idt_ptr;


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

extern void	asm_cpu_default_interrupt(void);

extern void	asm_divide_by_zero(void);
extern void	asm_debug(void);
extern void	asm_non_maskable_interrupt(void);
extern void	asm_breakpoint(void);
extern void	asm_overflow(void);
extern void	asm_bound_range_exceeded(void);
extern void	asm_invalid_opcode(void);
extern void	asm_no_device(void);
extern void	asm_double_fault(void);
extern void	asm_fpu_seg_overrun(void);
extern void	asm_invalid_tss(void);
extern void	asm_seg_no_present(void);
extern void	asm_stack_seg_fault(void);
extern void	asm_general_protect_fault(void);
extern void	asm_page_fault(void);
extern void	asm_fpu_floating_point_exep(void);
extern void	asm_alignment_check(void);
extern void	asm_machine_check(void);
extern void	asm_simd_fpu_fp_exception(void);
extern void	asm_virtualize_exception(void);
extern void	asm_security_exception(void);

extern void	asm_default_interrupt(void);
extern void	asm_default_pic_master_interrupt(void);
extern void	asm_default_pic_slave_interrupt(void);

extern void	asm_pit_isr(void);
extern void	asm_keyboard_isr(void);

void		init_idt(void)
{
	for (int i = 0; i < 32; i++)
		initialize_idt_seg(
				i,
				(u32)&asm_cpu_default_interrupt,
				0x8,
				INTGATE);

	for (int i = 32; i < IDT_SIZE; i++)
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
	initialize_idt_seg(1, (u32)&asm_debug, 0x8, INTGATE);
	initialize_idt_seg(2, (u32)&asm_non_maskable_interrupt, 0x8, INTGATE);
	initialize_idt_seg(3, (u32)&asm_breakpoint, 0x8, INTGATE);
	initialize_idt_seg(4, (u32)&asm_overflow, 0x8, INTGATE);
	initialize_idt_seg(5, (u32)&asm_bound_range_exceeded, 0x8, INTGATE);
	initialize_idt_seg(6, (u32)&asm_invalid_opcode, 0x8, INTGATE);
	initialize_idt_seg(7, (u32)&asm_no_device, 0x8, INTGATE);

	initialize_idt_seg(8, (u32)&asm_double_fault, 0x8, INTGATE);

	initialize_idt_seg(9, (u32)&asm_fpu_seg_overrun, 0x8, INTGATE);

	initialize_idt_seg(10, (u32)&asm_invalid_tss, 0x8, INTGATE);
	initialize_idt_seg(11, (u32)&asm_seg_no_present, 0x8, INTGATE);
	initialize_idt_seg(12, (u32)&asm_stack_seg_fault, 0x8, INTGATE);
	initialize_idt_seg(13, (u32)&asm_general_protect_fault, 0x8, INTGATE);
	initialize_idt_seg(14, (u32)&asm_page_fault, 0x8, INTGATE);

	initialize_idt_seg(16, (u32)&asm_fpu_floating_point_exep, 0x8, INTGATE);
	initialize_idt_seg(17, (u32)&asm_alignment_check, 0x8, INTGATE);
	initialize_idt_seg(18, (u32)&asm_machine_check, 0x8, INTGATE);

	initialize_idt_seg(19, (u32)&asm_simd_fpu_fp_exception, 0x8, INTGATE);
	initialize_idt_seg(20, (u32)&asm_virtualize_exception, 0x8, INTGATE);
	initialize_idt_seg(30, (u32)&asm_security_exception, 0x8, INTGATE);


	initialize_idt_seg(32, (u32)&asm_pit_isr, 0x8, INTGATE);
	initialize_idt_seg(33, (u32)&asm_keyboard_isr, 0x8, INTGATE);

	idt_ptr.limit = IDT_SIZE << 3;
	idt_ptr.base = IDT_ADDRESS;

	asm("lidt (idt_ptr)");
}
