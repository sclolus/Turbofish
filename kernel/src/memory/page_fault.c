
#include "memory_manager.h"

#include "libft.h"

/*
 * -> P 1 bit Present, When set, the page fault was caused by a page-protection
 * violation. When not set, it was caused by a non-present page.
 * -> W 1 bit Write, When set, the page fault was caused by a page write. When
 * not set, it was caused by a page read.
 * -> U 1 bit User, When set, the page fault was caused while CPL = 3. This
 * does not necessarily mean that the page fault was a privilege violation.
 * -> R 1 bit Reserved write, When set, the page fault was caused by writing
 * in a reserved field.
 * -> I 1 bit Instruction Fetch, When set, the page fault was caused by an
 * instruction fetch.
 */
#define BIT_PRESENT	(1 << 0)
#define BIT_WRITE	(1 << 1)
#define BIT_USER	(1 << 2)
#define BIT_RES_WRITE	(1 << 3)
#define BIT_I_FETCH	(1 << 4)

static u32 g_page_fault_count = 0;

int	page_fault_handler(u32 err_reg, u32 fault_addr)
{
	int ret;

	(void)err_reg;
	g_page_fault_count++;
	/*
	 * test if fault address is in the VALLOC area field
	 */
	if ((fault_addr & VALLOC_MASK) == VALLOC_SPACE)
		ret = v_assign_phy_area(fault_addr);
	else
		ret = -1;

	if (ret < 0)
		return -1;
	return 0;
}

u32	get_nb_page_fault(void)
{
	return g_page_fault_count;
}
