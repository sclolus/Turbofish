
#ifndef __MEMORY_MANAGER_H__
# define __MEMORY_MANAGER_H__

#include "i386_type.h"

#include "buddy/buddy.h"

#define VALLOC_SPACE	0xC8000000
#define VALLOC_MASK	0xF8000000

int			page_fault_handler(u32 err_reg, u32 fault_addr);

extern void		asm_paging_set_page_directory_address(
				ptr_32 *page_directory_address);

int			map_address(
			u32 virt_addr,
			u32 page_req,
			u32 phy_addr,
			enum mem_space space);

/*
 * debug functions
 */
void			get_anotomie_of(void *virt_addr, size_t size);

/*
 * kernel public functions
 */
void			*kmmap(size_t size);
void			*vmmap(size_t size);

int			kmunmap(void *virt_addr);
int			vmunmap(void *virt_addr, size_t size);

int			v_assign_phy_area(u32 fault_addr);

#endif
