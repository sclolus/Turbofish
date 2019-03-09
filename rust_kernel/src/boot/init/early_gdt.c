
#include "early_gdt.h"

/*
 * Create a new GDT segment and put it int the right place
 */
void create_gdt_segment(
	struct gdt_info gdt_info,
	u8 idx, u32 base, u32 limit, u8 access, u8 flags)
{
	struct gdt_segment *seg = &(gdt_info.gdt_location->segments[idx]);

	seg->limit_0_15 = limit & 0xffff;
	seg->limit_16_19 = (limit >> 16) & 0xf;
	seg->base_0_15 = base & 0xffff;
	seg->base_16_23 = (base >> 16) & 0xff;
	seg->base_24_31 = (base >> 24) & 0xff;
	seg->access_bytes = access;
	seg->flags = flags;
}

/*
 * Make a basic GDT for you
 */
struct gdt_info	gdt_new(void)
{
	struct gdt_info gdt_info;

	gdt_info.gdt_size = MAX_GDT_ENTRIES;
	gdt_info.gdt_location = (struct gdt *)GDT_LOCATION;

	create_gdt_segment(gdt_info, 0, 0, 0, 0, 0);

	create_gdt_segment(gdt_info, 1, 0, 0xfffff,
			PR | SYSTEM_HOLDER | EXECUTABLE,
			SIZE | GRANULARITY);
	create_gdt_segment(gdt_info, 2, 0, 0xfffff,
			PR | SYSTEM_HOLDER | READ_WRITE,
			SIZE | GRANULARITY);
	create_gdt_segment(gdt_info, 3, 0, 0xfffff,
			PR | SYSTEM_HOLDER | READ_WRITE,
			SIZE | GRANULARITY);
	create_gdt_segment(gdt_info, 4, 0, 0xfffff,
			PR | SYSTEM_HOLDER | EXECUTABLE | DPL,
			SIZE | GRANULARITY);
	create_gdt_segment(gdt_info, 5, 0, 0xfffff,
			PR | SYSTEM_HOLDER | READ_WRITE | DPL,
			SIZE | GRANULARITY);
	create_gdt_segment(gdt_info, 6, 0, 0xfffff,
			PR | SYSTEM_HOLDER | READ_WRITE | DPL,
			SIZE | GRANULARITY);

	return gdt_info;
}
