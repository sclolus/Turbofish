
#ifndef __DYNAMIC_ALLOCATOR_H__
# define __DYNAMIC_ALLOCATOR_H__

# include "i386_type.h"

/*
 * Kernel K-Family memory helpers
 */
void	*kmalloc(size_t size);
int	kfree(void *addr);
size_t	ksize(void *addr);

void	*kcalloc(size_t count, size_t size);
void	*krealloc(void *addr, size_t size);

void	kshow_alloc_mem(void);
void	kshow_alloc_mem_ex(void);

/*
 * Kernel V-Family memory helpers
 */
void	*valloc(size_t size);
int	vfree(void *addr);
size_t	vsize(void *addr);

u32	get_nb_page_fault(void);

#endif
