
#ifndef __EARLY_GDT_H__
# define __EARLY_GDT_H__

#include "i386_type.h"

#define GDT_LOCATION 0x800
#define MAX_GDT_ENTRIES 128

// ACCESS BYTES DESCRIPTION
#define ACCESSED 1 // WAS IT ACCESSED
#define READ_WRITE (1 << 1) // FOR DATA SEGMENT IS WRITING ALLOWED ?
#define GROWTH_DIRECTION (1 << 2) // FOR DATA SEGMENT: TO LOWER OR TO HIGHER ADDRESS. FOR TEXT SEGMENT : CAN IT BE EXECTUDED WITH HIGHER PRIVILEDGE ?
#define EXECUTABLE (1 << 3)
#define SYSTEM_HOLDER (1 << 4) // IS IT DATA/CODE ? (1) OR IS IT JUST SOME SYSTEM INFORMATION HOLDER (0)
#define DPL ((1 << 5) | (1 << 6)) // DESCRIPTOR PRIVILEGE LEVEL (RING)
#define PR (1 << 7) // PRESENT IN MEMORY RIGHT NOW ?

// FLAG DESCRIPTION
#define V 1 // AVAILABLE TO USE FOR SYSTEM SOFTWARE ?
#define LONGMODE (1 << 1) // IS IT A 64 BIT MODE SEGMENT ?
#define SIZE (1 << 2) // (0) 16 BIT (1) FOR 32 BIT PROTECTED
#define GRANULARITY (1 << 3) // LIMIT IS IN 0 = BYTES, 1 = PAGES OF LIMIT 4096 BYTES EACH

struct __attribute__ ((packed)) gdt_segment {
	u16 limit_0_15;
	u16 base_0_15;
	u8 base_16_23;
	u8 access_bytes;
	u8 limit_16_19: 4;
	u8 flags: 4;
	u8 base_24_31;
};

struct __attribute__ ((packed)) gdt {
	struct gdt_segment segments[128];
};

struct __attribute__ ((packed)) gdt_info {
	u16 gdt_size;
	struct gdt *gdt_location;
};

struct gdt_info gdt_new(void);

struct gdt_info create_gdt_segment(struct gdt_info gdt_info, u8 idx, u32 base, u32 limit, u8 access, u8 flags);

#endif
