
#ifndef __MEMORY_MANAGER_H__
# define __MEMORY_MANAGER_H__

#include "i386_type.h"

#define MAP_FAILED	0xFFFFFFFF

void			init_kernel_alloc_frames(void);
void			*alloc_frames(u32 page_request);
int			free_frames(void *addr);
u32			count_frames(void);

u32			paginate(
			u32 directory,
			u32 segment,
			u32 page_request,
			u32 address);

int			unpaginate(
			u32 directory,
			u32 segment,
			u32 page_request);

int			create_directory(u32 directory);

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
