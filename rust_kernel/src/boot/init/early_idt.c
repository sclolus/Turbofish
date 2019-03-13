
#include "early_idt.h"

#include "libft.h"

extern void	isr_cpu_default_interrupt(void);

extern void	isr_divide_by_zero(void);
extern void	isr_debug(void);
extern void	isr_non_maskable_interrupt(void);
extern void	isr_breakpoint(void);
extern void	isr_overflow(void);
extern void	isr_bound_range_exceeded(void);
extern void	isr_invalid_opcode(void);
extern void	isr_no_device(void);
extern void	isr_double_fault(void);
extern void	isr_fpu_seg_overrun(void);
extern void	isr_invalid_tss(void);
extern void	isr_seg_no_present(void);
extern void	isr_stack_seg_fault(void);
extern void	isr_general_protect_fault(void);
extern void	isr_page_fault(void);
extern void	isr_fpu_floating_point_exep(void);
extern void	isr_alignment_check(void);
extern void	isr_machine_check(void);
extern void	isr_simd_fpu_fp_exception(void);
extern void	isr_virtualize_exception(void);
extern void	isr_security_exception(void);

void initialize_idt_seg(struct idt_segment *segment, u32 fn_addr, u16 selector, u16 type)
{
	segment->offset0_15 = fn_addr & 0xFFFF;
	segment->select = selector;
	segment->type = type;
	segment->offset16_31 = (fn_addr & 0xFFFF0000) >> 16;
}

void init_early_idt(struct idt_info *idt_info)
{
	idt_info->idt_limit = IDT_SIZE << 3;
	idt_info->idt_location = (struct idt *)IDT_ADDRESS;

	ft_memset(idt_info->idt_location->segments, 0, sizeof(struct idt_segment) * IDT_SIZE);

	for (int i = 0x0; i < 0x20; i++)
		initialize_idt_seg(
				&(idt_info->idt_location->segments[i]),
				(u32)&isr_cpu_default_interrupt,
				0x8,
				INTGATE32);

	initialize_idt_seg(&(idt_info->idt_location->segments[0x00]), (u32)&isr_divide_by_zero, 0x8, INTGATE32);
	initialize_idt_seg(&(idt_info->idt_location->segments[0x01]), (u32)&isr_debug, 0x8, TRAPGATE32);
	initialize_idt_seg(&(idt_info->idt_location->segments[0x02]), (u32)&isr_non_maskable_interrupt, 0x8, INTGATE32);
	initialize_idt_seg(&(idt_info->idt_location->segments[0x03]), (u32)&isr_breakpoint, 0x8, TRAPGATE32);
	initialize_idt_seg(&(idt_info->idt_location->segments[0x04]), (u32)&isr_overflow, 0x8, TRAPGATE32);
	initialize_idt_seg(&(idt_info->idt_location->segments[0x05]), (u32)&isr_bound_range_exceeded, 0x8, INTGATE32);
	initialize_idt_seg(&(idt_info->idt_location->segments[0x06]), (u32)&isr_invalid_opcode, 0x8, INTGATE32);
	initialize_idt_seg(&(idt_info->idt_location->segments[0x07]), (u32)&isr_no_device, 0x8, INTGATE32);
	initialize_idt_seg(&(idt_info->idt_location->segments[0x08]), (u32)&isr_double_fault, 0x8, INTGATE32);
	initialize_idt_seg(&(idt_info->idt_location->segments[0x09]), (u32)&isr_fpu_seg_overrun, 0x8, INTGATE32);
	initialize_idt_seg(&(idt_info->idt_location->segments[0x0A]), (u32)&isr_invalid_tss, 0x8, INTGATE32);
	initialize_idt_seg(&(idt_info->idt_location->segments[0x0B]), (u32)&isr_seg_no_present, 0x8, INTGATE32);
	initialize_idt_seg(&(idt_info->idt_location->segments[0x0C]), (u32)&isr_stack_seg_fault, 0x8, INTGATE32);
	initialize_idt_seg(&(idt_info->idt_location->segments[0x0D]), (u32)&isr_general_protect_fault, 0x8, INTGATE32);
	initialize_idt_seg(&(idt_info->idt_location->segments[0x0E]), (u32)&isr_page_fault, 0x8, INTGATE32);

	initialize_idt_seg(&(idt_info->idt_location->segments[0x10]), (u32)&isr_fpu_floating_point_exep, 0x8, INTGATE32);
	initialize_idt_seg(&(idt_info->idt_location->segments[0x11]), (u32)&isr_alignment_check, 0x8, INTGATE32);
	initialize_idt_seg(&(idt_info->idt_location->segments[0x12]), (u32)&isr_machine_check, 0x8, INTGATE32);
	initialize_idt_seg(&(idt_info->idt_location->segments[0x13]), (u32)&isr_simd_fpu_fp_exception, 0x8, INTGATE32);
	initialize_idt_seg(&(idt_info->idt_location->segments[0x14]), (u32)&isr_virtualize_exception, 0x8, INTGATE32);

	initialize_idt_seg(&(idt_info->idt_location->segments[0x1E]), (u32)&isr_security_exception, 0x8, INTGATE32);
}
