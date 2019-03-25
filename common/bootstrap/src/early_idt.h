
#ifndef __EARLY_IDT_H__
# define __EARLY_IDT_H__

#include "i386_type.h"

/*
 * IDT region: 0x1000 -> 0x1800 (IDT_SIZE * SIZE(IDT_SEGMENT) = 256 * 8 = 2000 = 0x800)
 */
#define IDT_SIZE 256
#define IDT_ADDRESS 0x1000
#define INTGATE32 0x8E00
#define TRAPGATE32 0x8F00

struct __attribute__ ((packed)) idt_segment {
	u16 offset0_15;
	u16 select;
	u16 type;
	u16 offset16_31;
};

struct __attribute__ ((packed)) idt {
	struct idt_segment segments[IDT_SIZE];
};

struct __attribute__ ((packed)) idt_info {
	u16 idt_limit;
	struct idt *idt_location;
};

void initialize_idt_seg(struct idt_segment *segment, u32 fn_addr, u16 selector, u16 type);

void init_early_idt(struct idt_info *idt_info);

#endif
