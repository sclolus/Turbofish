
#ifndef __MEMORY_MANAGER_H__
# define __MEMORY_MANAGER_H__

#include "i386_type.h"

#define MAP_FAILED	0xFFFFFFFF

enum mem_space {
	kernel_space,
	user_space
};

// virtual map internal functions
void			init_virtual_map(void);
void			*get_pages(u32 page_request, enum mem_space space);
u32			free_pages(void *addr, enum mem_space space);

// physical map internal functions
void			init_physical_map(void);
int			mark_physical_area(void *addr, u32 page_request);
void			*get_physical_addr(u32 page_request);
int			drop_physical_addr(void *addr);

// kernel public function
void			*kmmap(u32 page_req);
int			kmunmap(void *addr);

void			kfree(void *ptr);
void			*kmalloc(size_t size);
void			*kcalloc(size_t count, size_t size);
void			*krealloc(void *ptr, size_t size);
void			*kreallocf(void *ptr, size_t size);
void			*kreallocarray(void *ptr, size_t nmemb, size_t size);
void			*kvalloc(size_t size);
void			kshow_alloc_mem(void);
void			kshow_alloc_mem_ex(void);

#endif
