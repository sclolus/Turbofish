
#include "memory_manager.h"

#include "libft.h"

void	page_fault_handler(u32 fault_addr)
{
	int ret;

	if ((fault_addr & VALLOC_MASK) == VALLOC_SPACE)
		ret = v_assign_phy_area(fault_addr);
	else
		ret = -1;

	if (ret < 0) {
		printk("{red}PaGe FaUlT at %p ! Cannot do anything{eoc}\n",
				(void *)fault_addr);
		while(1);
	}
}
