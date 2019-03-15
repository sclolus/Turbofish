
#include "memory_manager.h"

#include "i386_type.h"
#include "system.h"
#include "libft.h"
#include "vesa.h"

#define MAX_DIRECTORY_SEG		1024
#define PAGE_DIRECTORY_0_ADDR		0x10000
#define PAGE_TABLE_0_ADDR		0x800000

#define MAX_PAGE_TABLE_SEG		1024
#define OFFSET				4096

#define MINIMUM_MEMORY			(1 << 25)

/*
 * Field type:
 * - P indicate if is a physical memory area
 * - RW indicate if page is readable and writable
 * - US indicate if simple user can use that page
 * - PWT cache management (no founded)
 * - PCB cache management (no founded)
 * - D indicate if the page has been written. page only
 * - PS indicate if pages contained are 4mo size instead of 4ko.
 * page directory only
 */
#define P_IS_PHYSIC_MEMORY		(1 << 0)
#define RW_IS_READ_AND_WRITE		(1 << 1)
#define US_IS_USER_USABLE		(1 << 2)
#define PWT_CACHE_MANAGE_1		(1 << 3)
#define PCB_CACHE_MANAGE_2		(1 << 4)
#define D_IS_WRITED_TABLE		(1 << 6)
#define PS_IS_4MO_SIZE_DIRECTORY	(1 << 7)

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

struct __attribute__ ((packed)) page_table_area {
	struct page_table page_table[MAX_DIRECTORY_SEG];
};

struct __attribute__ ((packed)) page_directory {
	struct page_directory_seg seg[MAX_DIRECTORY_SEG];
};

/*
 * get the physical page address associated with a virtual address
 * @return: VOID PTR
 * 	a valid physical address on success
 * 	MAP_FAILED on error
 */
static void		*virt_to_physical_addr(u32 virt_addr)
{
	struct page_table_seg	*pt;
	u32			phy_addr;

	if (virt_addr & PAGE_MASK) {
		eprintk("%s: Unexpected offset, virt_addr = %p\n",
				__func__, virt_addr);
		return (void *)MAP_FAILED;
	}
	/*
	 * conversion from virt_add 0 -> 4go to table pages 4mo -> 8mo
	 */
	pt = (struct page_table_seg *)((virt_addr >> 10) + PAGE_TABLE_0_ADDR + 0xc0000000);

	phy_addr = pt->physical_address4_20 & 0xFFFF;
	phy_addr <<= 4;
	phy_addr |= pt->physical_address0_3 & 0xF;
	phy_addr <<= 12;

	return (void *)phy_addr;
}

/*
 * convert a size to the number of page frame associated
 * @return: SIZE_T
 * 	the number of pages
 */
static inline size_t	size_to_page_requested(size_t size)
{
	return (size >> 12) + ((size & PAGE_MASK) ? 1 : 0);
}

/*
 * create a page table directory at index 'directory'
 * @return: INTEGER
 * 	0 on success
 */
static int		create_directory(u32 directory, enum mem_space space)
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

/*
* clobber memory to avoid optimizer re-ordering access before INVLPG,
* which may cause nasty bugs.
*/
static inline void	invlpg(void *m)
{
	asm volatile("invlpg (%0)" : : "b"(m) : "memory");
}

/*
 * write on the page table the physical address associated to virtual address,
 * on the size length.
 * @return: INTEGER
 * 	 0 on success
 * 	 -1 on error
 */
int			map_address(
			u32 virt_addr,
			u32 page_req,
			u32 phy_addr,
			enum mem_space space)
{
	struct page_table_seg *pt;

	if (virt_addr & PAGE_MASK) {
		eprintk("%s: Unexpected offset, virt_addr = %p\n",
				__func__, virt_addr);
		return -1;
	}

	/*
	 * conversion from virt_add 0 -> 4go to table pages 4mo -> 8mo
	 */
	pt = (struct page_table_seg *)((virt_addr >> 10) + PAGE_TABLE_0_ADDR + 0xc0000000);

	for (u32 i = 0; i < page_req; i++) {
		pt->type = P_IS_PHYSIC_MEMORY | RW_IS_READ_AND_WRITE;
		if (space == user_space)
			pt->type |= US_IS_USER_USABLE;
		pt->cache = 0;
		pt->available = 0;
		pt->physical_address0_3 = (phy_addr >> 12) & 0xF;
		pt->physical_address4_20 = (phy_addr >> 12) >> 4;

		invlpg((void *)virt_addr);
		virt_addr += PAGE_SIZE;

		pt++;
		phy_addr += OFFSET;
	}
	return 0;
}

/*
 * allocate continuous memory on the heap associated with physical address
 * @return: VOID PTR
 *	a valid physical address on success
 * 	MAP_FAILED on error
 */
void			*kmmap(size_t size)
{
	u32	res;
	void	*phy_addr;
	u32	page_req;

	page_req = size_to_page_requested(size);
	res = get_pages(page_req, kheap);
	if (res != MAP_FAILED) {
		phy_addr = get_physical_addr(page_req);
		if ((u32)phy_addr == MAP_FAILED) {
			eprintk("%s: out of physical memory\n", __func__);
			page_req = free_pages((void *)res, kheap);
			if (page_req == 0)
				eprintk("%s: Unexpected error\n", __func__);
			return (void *)MAP_FAILED;
		}
		map_address(
			res,
			page_req,
			(u32)phy_addr,
			kernel_space);
	}
	else
		eprintk("%s: out of virtual memory\n", __func__);
	return (void *)res;
}

/*
 * allocate a virtual address chunk without physical address associated
 * @return: VOID PTR
 *	a valid physical address on success
 * 	MAP_FAILED on error
 */
void			*vmmap(size_t size)
{
	return (void *)get_pages(size_to_page_requested(size), vheap);
}

/*
 * deallocate a virtual address and his continuous physical content
 * @return: INTEGER
 * 	0 on success
 * 	-1 on error
 */
int			kmunmap(void *virt_addr)
{
	u32	*pt;
	u32	page_req;
	void	*phy_addr;

	if ((u32)virt_addr & PAGE_MASK) {
		eprintk("%s: unexpected offset, virt_addr = %p\n",
				__func__, virt_addr);
		return -1;
	}

	page_req = free_pages(virt_addr, kheap);
	if (page_req == 0) {
		eprintk("%s: Unexpected size, virt_addr = %p\n",
				__func__, virt_addr);
		return -1;
	}

	phy_addr = virt_to_physical_addr((u32)virt_addr);

	if (drop_physical_addr(phy_addr) == 0)
		eprintk("%s: Unexpected error\n", __func__);

	pt = (u32 *)(((u32)virt_addr >> 10) + PAGE_TABLE_0_ADDR + 0xc0000000);

	/*
	 * UNMAP page table
	 */
	for (u32 i = 0; i < page_req; i++)
	{
		*pt++ = 0;
		invlpg(virt_addr);
		virt_addr += PAGE_SIZE;
	}
	return 0;
}

/*
 * deallocate a virtual address and all the physical address associated
 * @return: INTEGER
 * 	0 on success
 * 	-1 on error
 * NB: assuming size and virt_addr are multiple of PAGE_SIZE
 */
int			vmunmap(void *virt_addr, size_t size)
{
	u32 *pt;
	u32 page_req;
	u32 phy_addr;

	if ((u32)virt_addr & PAGE_MASK) {
		eprintk("%s: Unexpected offset, virt_addr = %p\n",
				__func__, virt_addr);
		return -1;
	}

	if (size & PAGE_MASK) {
		eprintk("%s: Unexpected size, size = %u\n",
				__func__, size);
		return -1;
	}

	page_req = free_pages(virt_addr, vheap);
	if (page_req == 0) {
		eprintk("%s: Unable to found virt_addr record\n", __func__);
		return -1;
	}

	/*
	 * conversion from virt_add 0 -> 4go to table pages 4mo -> 8mo
	 */
	pt = (u32 *)(((u32)virt_addr >> 10) + PAGE_TABLE_0_ADDR + 0xc0000000);

	size >>= 12;
	for (size_t i = 0; i < size; i++) {

		/*
		 * assuming that address is the higher 20bits of page_table
		 */
		phy_addr = *pt & 0xFFFFF000;

		/*
		 * For VMALLOC, it's important to consult if phy_addr != 0
		 * Allocations may be done after the first page, so the physic
		 * address of the first page is 0 according to a constant set 0
		 * of unallocated pages
		 */
		if (phy_addr != 0) {
			if (drop_physical_addr((void *)phy_addr) == 0)
				eprintk("%s: Unexpected error at physic %p "
					"virtual %p\n",
					__func__, phy_addr, virt_addr);

			/*
			 * UNMAP page table
			 */
			*pt = 0;
			invlpg((void *)virt_addr);
		}
		virt_addr += PAGE_SIZE;
		pt++;
	}
	return 0;
}

/*
 * initialize all the paging system
 */
int	init_paging(u32 available_memory, u32 *vesa_framebuffer)
{
	u32 res;
	int i;

	/*
	 * check minimum vital memory
	 */
	if (available_memory <= MINIMUM_MEMORY) {
		eprintk("%s: Not enough memory: got %uo, minimum %uo\n",
				__func__, available_memory, MINIMUM_MEMORY);
		return -1;
	}

	/*
	 * Creation of first mo page directory (4mo will be assigned)
	 */
	i = 0;
	for (; i < 1; i++)
		create_directory(i, kernel_space);

	/*
	 * creation of user page directory
	 */
	for (; i < 3 * MAX_DIRECTORY_SEG / 4; i++)
		create_directory(i, user_space);

	/*
	 * creation of kernel page directory
	 */
	for (; i < MAX_DIRECTORY_SEG; i++)
		create_directory(i, kernel_space);

	/*
	 * clean all pages tables
	 */
	bzero((void *)PAGE_TABLE_0_ADDR, sizeof(struct page_table_area));

	/*
	 * initialize virtual memory map
	 */
	init_virtual_map();

	/*
	 * initialize physical memory map
	 */
	init_physical_map((void *)available_memory);


	/*
	 * Map the first 1mo on identity mapping
	 */
	res = get_pages(256, first_mo);
	map_address(res, 256, 0x0, kernel_space);

	/*
	 * mapping of first 4mo, GDT, IDT, page directory, kernel, stack
	 */
	res = get_pages(MAX_PAGE_TABLE_SEG, reserved);
	map_address(res, MAX_PAGE_TABLE_SEG, 0x0, kernel_space);
	mark_physical_area((void *)0x0, MAX_PAGE_TABLE_SEG);

	/*
	 * mapping of next 4mo, stack and memory map
	 */
	res = get_pages(MAX_PAGE_TABLE_SEG, reserved);
	map_address(
			res,
			MAX_PAGE_TABLE_SEG,
			0x400000,
			kernel_space);
	mark_physical_area((void *)0x400000, MAX_PAGE_TABLE_SEG);

	/*
	 * mapping of next 4mo, pages list
	 */
	res = get_pages(MAX_PAGE_TABLE_SEG, reserved);
	map_address(
			res,
			MAX_PAGE_TABLE_SEG,
			0x800000,
			kernel_space);
	mark_physical_area((void *)0x800000, MAX_PAGE_TABLE_SEG);

	if (vesa_framebuffer != NULL) {
		/*
		 * mapping of next 4mo, double frame buffer
		 */
		res = get_pages(MAX_PAGE_TABLE_SEG, reserved);
		map_address(
				res,
				MAX_PAGE_TABLE_SEG,
				DB_FRAMEBUFFER_ADDR - 0xc0000000,
				kernel_space);
		mark_physical_area(
				(void *)DB_FRAMEBUFFER_ADDR - 0xc0000000,
				MAX_PAGE_TABLE_SEG);

		/*
		 * mapping of LFB VBE
		 */
		res = get_pages(MAX_PAGE_TABLE_SEG, reserved);
		map_address(
				res,
				MAX_PAGE_TABLE_SEG,
				*vesa_framebuffer,
				kernel_space);
		mark_physical_area(
				(void *)*vesa_framebuffer,
				MAX_PAGE_TABLE_SEG);
		*vesa_framebuffer = res;
		init_gdt(*vesa_framebuffer);
	}

	/*
	 * store page directory address in CR3 register
	 */
	asm_paging_set_page_directory_address(
			(ptr_32 *)PAGE_DIRECTORY_0_ADDR);

	/*
	 * launch paging: Already launched
	 */
	//asm_paging_enable();
	return 0;
}

/*
 * debug function, describe physical segments pointed by a virtual address
 * while size
 */
void			get_anotomie_of(void *virt_addr, size_t size)
{
	u32 _virt_addr;
	u32 _virt_addr_end;

	_virt_addr = (u32)virt_addr;
	_virt_addr_end = _virt_addr + size_to_page_requested(size) * PAGE_SIZE;

	while (_virt_addr < _virt_addr_end) {
		printk("phy_addr = %p\n", virt_to_physical_addr(_virt_addr));
		_virt_addr += PAGE_SIZE;
	}
}
