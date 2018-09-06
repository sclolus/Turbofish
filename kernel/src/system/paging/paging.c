
#include "i386_type.h"
#include "base_system.h"
#include "libft.h"
#include "vesa_graphic.h"

#define MAX_DIRECTORY_SEG	1024
#define MAX_PAGE_TABLE_SEG	1024
#define OFFSET			4096

// type field
#define P_IS_PHYSIC_MEMORY              (1 << 0) // P indicate if is a physical memory area
#define RW_IS_READ_AND_WRITE            (1 << 1) // RW indicate if page is readable and writable
#define US_IS_USER_USABLE               (1 << 2) // US indicate if simple user can use that page
#define PWT_CACHE_MANAGE_1              (1 << 3) // PWT cache management (no founded)
#define PCB_CACHE_MANAGE_2              (1 << 4) // PCB cache management (no founded)
#define D_IS_WRITED_TABLE               (1 << 6) // D indicate if the page has been written. page only
#define PS_IS_4MO_SIZE_DIRECTORY        (1 << 7) // PS indicate if pages contained are 4mo size instead of 4ko. page directory only

struct __attribute__ ((packed)) page_directory_seg {
	u8 type;
	u8 size			:1;
	u8 available		:3;
	u8 physical_address0_3	:4;
	u16 physical_address4_20;
};

struct __attribute__ ((packed)) page_table_seg {
	u8 type;
	u8 cache		:1;
	u8 available		:3;
	u8 physical_address0_3	:4;
	u16 physical_address4_20;
};

struct __attribute__ ((packed)) page_table {
	struct page_table_seg seg[MAX_PAGE_TABLE_SEG];
};

struct __attribute__ ((packed)) page_directory {
	struct page_directory_seg seg[MAX_DIRECTORY_SEG];
};

#define PAGE_DIRECTORY_0_ADDR 0x1000
#define PAGE_TABLE_0_ADDR 0x400000

static u32 g_pd_nb = 0;

u32	map_address(u32 address)
{
	struct page_directory	*pd;
	struct page_table	*pt;
	u32			linear_address;

	pd = (struct page_directory *)PAGE_DIRECTORY_0_ADDR;
	pt = (struct page_table *)(PAGE_TABLE_0_ADDR
			+ (g_pd_nb * sizeof(struct page_table)));

	pd->seg[g_pd_nb].type = P_IS_PHYSIC_MEMORY | RW_IS_READ_AND_WRITE;
	pd->seg[g_pd_nb].size = 0;
	pd->seg[g_pd_nb].available = 0;
	pd->seg[g_pd_nb].physical_address0_3 = ((u32)pt >> 12) & 0xF;
	pd->seg[g_pd_nb].physical_address4_20 = ((u32)pt >> 12) >> 4;

	for (int i = 0; i < MAX_PAGE_TABLE_SEG; i++) {
		pt->seg[i].type = P_IS_PHYSIC_MEMORY | RW_IS_READ_AND_WRITE;
		pt->seg[i].cache = 0;
		pt->seg[i].available = 0;
		pt->seg[i].physical_address0_3 = (address >> 12) & 0xF;
		pt->seg[i].physical_address4_20 = (address >> 12) >> 4;
		address += OFFSET;
	}
	linear_address = (g_pd_nb & 0x3FF) << 22;
	g_pd_nb++;

	return (linear_address);
}

void init_paging(void)
{
	ft_bzero((void *)PAGE_DIRECTORY_0_ADDR, sizeof(struct page_directory));
	ft_bzero((void *)PAGE_TABLE_0_ADDR, sizeof(struct page_table));

	map_address(0x0);
	map_address(0x400000);

// TODO Management of LFB with pagination is very dirty.
// We have not to change vesa_mode_info.framebuffer !!!
	u32 new_lfb = map_address((u32)g_graphic_ctx.vesa_mode_info.framebuffer);
	init_GDT((void *)new_lfb);
	g_graphic_ctx.vesa_mode_info.framebuffer = (void *)new_lfb;

	asm_paging_set_page_directory_address(
			(ptr_32 *)PAGE_DIRECTORY_0_ADDR);
	asm_paging_enable();
}
