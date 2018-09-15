
#ifndef __DYNAMIC_ALLOCATOR_H__
# define __DYNAMIC_ALLOCATOR_H__

# include "i386_type.h"

/*
 * Kernel K-Family memory helpers
 */
void	*kmalloc(size_t size);
int	kfree(void *ptr);
size_t	ksize(void *ptr);

void	*kcalloc(size_t count, size_t size);
void	*krealloc(void *ptr, size_t size);

void	kshow_alloc_mem(void);
void	kshow_alloc_mem_ex(void);

/*
 * Kernel V-Family memory helpers
 */
void	*valloc(size_t size);
int	vfree(void *ptr);
size_t	vsize(void *ptr);

#endif
