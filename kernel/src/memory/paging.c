
#include "i386_type.h"
#include "base_system.h"
#include "libft.h"
#include "vesa_graphic.h"
#include "memory_manager.h"

#define MAX_DIRECTORY_SEG		1024
#define PAGE_DIRECTORY_0_ADDR		0x1000
#define PAGE_TABLE_0_ADDR		0x400000

#define MAX_PAGE_TABLE_SEG		1024
#define OFFSET				4096

// type field
#define P_IS_PHYSIC_MEMORY		(1 << 0) // P indicate if is a physical memory area
#define RW_IS_READ_AND_WRITE		(1 << 1) // RW indicate if page is readable and writable
#define US_IS_USER_USABLE		(1 << 2) // US indicate if simple user can use that page
#define PWT_CACHE_MANAGE_1		(1 << 3) // PWT cache management (no founded)
#define PCB_CACHE_MANAGE_2		(1 << 4) // PCB cache management (no founded)
#define D_IS_WRITED_TABLE		(1 << 6) // D indicate if the page has been written. page only
#define PS_IS_4MO_SIZE_DIRECTORY	(1 << 7) // PS indicate if pages contained are 4mo size instead of 4ko. page directory only

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

static void	*virt_to_physical_addr(u32 virt_addr)
{
	struct page_table_seg	*pt;
	u32			phy_addr;

	if (virt_addr & 0xFFF) {
		eprintk("%s: Unexpected offset, virt_addr = %p\n", virt_addr);
		return (void *)MAP_FAILED;
	}
	// conversion from virt_add 0 -> 4go to table pages 4mo -> 8mo
	pt = (struct page_table_seg *)((virt_addr >> 10) + PAGE_TABLE_0_ADDR);

	phy_addr = pt->physical_address4_20 & 0xFFFF;
	phy_addr <<= 4;
	phy_addr |= pt->physical_address0_3 & 0xF;
	phy_addr <<= 12;

	return (void *)phy_addr;
}

static int	create_directory(u32 directory, enum mem_space space)
{
	struct page_directory	*pd;
	struct page_table	*pt;

	pd = (struct page_directory *)PAGE_DIRECTORY_0_ADDR;
	pt = (struct page_table *)(PAGE_TABLE_0_ADDR
			+ (directory * sizeof(struct page_table)));

	pd->seg[directory].type = P_IS_PHYSIC_MEMORY | RW_IS_READ_AND_WRITE;
	if (space == user_space)
		pd->seg[directory].type |= US_IS_USER_USABLE;
	pd->seg[directory].size = 0;
	pd->seg[directory].available = 0;
	pd->seg[directory].physical_address0_3 = ((u32)pt >> 12) & 0xF;
	pd->seg[directory].physical_address4_20 = ((u32)pt >> 12) >> 4;
	return 0;
}

static int	map_address(
		u32 virt_addr,
		u32 page_req,
		u32 phy_addr,
		enum mem_space space)
{
	struct page_table_seg *pt;

	if (virt_addr & 0xFFF) {
		eprintk("%s: Unexpected offset, virt_addr = %p\n", virt_addr);
		return -1;
	}
	// conversion from virt_add 0 -> 4go to table pages 4mo -> 8mo
	pt = (struct page_table_seg *)((virt_addr >> 10) + PAGE_TABLE_0_ADDR);

	for (u32 i = 0; i < page_req; i++)
	{
		pt->type = P_IS_PHYSIC_MEMORY | RW_IS_READ_AND_WRITE;
		if (space == user_space)
			pt->type |= US_IS_USER_USABLE;
		pt->cache = 0;
		pt->available = 0;
		pt->physical_address0_3 = (phy_addr >> 12) & 0xF;
		pt->physical_address4_20 = (phy_addr >> 12) >> 4;
		pt++;
		phy_addr += OFFSET;
	}
	return 0;
}

static int	unmap_address(u32 virt_addr, u32 page_req)
{
	u32 *pt;

	if (virt_addr & 0xFFF) {
		eprintk("%s: Unexpected offset, virt_addr = %p\n", virt_addr);
		return -1;
	}
	pt = (u32 *)((virt_addr >> 10) + PAGE_TABLE_0_ADDR);

	for (u32 i = 0; i < page_req; i++)
		*pt++ = 0;
	return 0;
}

void		*kmmap(size_t size)
{
	void	*virt_addr;
	void	*phy_addr;
	u32	page_req;

	page_req = (size >> 12) + ((size & 0xFFF)  ? 1 : 0);
	virt_addr = get_pages(page_req, kernel_space);
	if ((u32)virt_addr != MAP_FAILED)
	{
		phy_addr = get_physical_addr(page_req);
		if ((u32)phy_addr == MAP_FAILED)
		{
			eprintk("%s: out of physical memory\n", __func__);
			page_req = free_pages(virt_addr, kernel_space);
			if (page_req == 0)
				eprintk("%s: Unexpected error\n", __func__);
			return (void *)MAP_FAILED;
		}

		map_address(
			(u32)virt_addr,
			page_req,
			(u32)phy_addr,
			kernel_space);
	}
	else
		eprintk("%s: out of virtual memory\n", __func__);
	return virt_addr;
}

void		*vmmap(size_t size)
{
	void	*virt_addr;
	u32	page_req;

	page_req = (size >> 12) + ((size & 0xFFF)  ? 1 : 0);
	virt_addr = get_pages(page_req, kernel_space);
	printk("virt_addr = %p\n", virt_addr);
	if ((u32)virt_addr != MAP_FAILED)
	{
		int ret = write_multiple_physical_addr(
				page_req,
				virt_addr,
				&map_address);
		if (ret == -1)
		{
			eprintk("%s: out of physical memory\n", __func__);
			page_req = free_pages(virt_addr, kernel_space);
			if (page_req == 0)
				eprintk("%s: Unexpected error\n", __func__);
			return (void *)MAP_FAILED;
		}
	}
	else
		eprintk("%s: out of virtual memory\n", __func__);
	return virt_addr;
}

int		kmunmap(void *virt_addr)
{
	u32			page_req;
	void			*phy_addr;

	page_req = free_pages(virt_addr, kernel_space);
	if (page_req == 0)
		return -1;

	phy_addr = virt_to_physical_addr((u32)virt_addr);

	if (drop_physical_addr(phy_addr) == 0)
		eprintk("%s: Unexpected error\n", __func__);

	return unmap_address((u32)virt_addr, page_req);
}

int		vmunmap(void *virt_addr)
{
	u32			page_req;
	u32			phy_addr;
	struct page_table_seg	*pt;

	if ((u32)virt_addr & 0xFFF) {
		eprintk("%s: Unexpected offset, virt_addr = %p\n"
				,__func__, virt_addr);
		return -1;
	}

	page_req = free_pages(virt_addr, kernel_space);
	if (page_req == 0)
		return -1;

	// conversion from virt_add 0 -> 4go to table pages 4mo -> 8mo
	pt = (struct page_table_seg *)
			(((u32)virt_addr >> 10) + PAGE_TABLE_0_ADDR);

	u32 i = 0;
	while (i < page_req)
	{
		phy_addr = pt->physical_address4_20 & 0xFFFF;
		phy_addr <<= 4;
		phy_addr |= pt->physical_address0_3 & 0xF;
		phy_addr <<= 12;

		u32 j = drop_physical_addr((void *)phy_addr);
		if (j == 0) {
			eprintk("%s: Unexpected error %p not founded ! V %p\n",
					__func__, phy_addr, virt_addr);
			return -1;
		}
		pt += j;
		i += j;
	}

	ft_bzero(virt_addr, page_req * 4);
	return 0;
}

void		init_paging(void)
{
	void	*virt_addr;
	int	i;

	// creation of kernel page directory
	i = 0;
	for (; i < MAX_DIRECTORY_SEG / 4; i++)
		create_directory(i, kernel_space);

	// creation of user page directory
	for (; i < MAX_DIRECTORY_SEG; i++)
		create_directory(i, user_space);

	// clean all pages tables
	ft_bzero((void *)PAGE_TABLE_0_ADDR, sizeof(struct page_table));

	// initialize virtual memory map
	init_virtual_map();

	// initialize physical memory map
	init_physical_map();

	// mapping of first 4mo, GDT, IDT, page directory, kernel, stack
	virt_addr = get_pages(MAX_PAGE_TABLE_SEG, kernel_space);
	map_address((u32)virt_addr, MAX_PAGE_TABLE_SEG, 0x0, kernel_space);
	mark_physical_area((void *)0x0, MAX_PAGE_TABLE_SEG);

	// mapping of next 4mo, pages list
	virt_addr = get_pages(MAX_PAGE_TABLE_SEG, kernel_space);
	map_address((
			u32)virt_addr,
			MAX_PAGE_TABLE_SEG,
			0x400000,
			kernel_space);
	mark_physical_area((void *)0x400000, MAX_PAGE_TABLE_SEG);

	// mapping of LFB VBE
	virt_addr = get_pages(MAX_PAGE_TABLE_SEG, kernel_space);
	map_address(
			(u32)virt_addr,
			MAX_PAGE_TABLE_SEG,
			(u32)g_graphic_ctx.vesa_mode_info.framebuffer,
			kernel_space);
	mark_physical_area(
			g_graphic_ctx.vesa_mode_info.framebuffer,
			MAX_PAGE_TABLE_SEG);
	init_GDT((void *)virt_addr);
	g_graphic_ctx.vesa_mode_info.framebuffer = (void *)virt_addr;

	// store page directory address in CR3 register
	asm_paging_set_page_directory_address(
			(ptr_32 *)PAGE_DIRECTORY_0_ADDR);

	// launch paging
	asm_paging_enable();
}
