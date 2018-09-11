
#ifndef __MEMORY_MANAGER_H__
# define __MEMORY_MANAGER_H__

#include "i386_type.h"

#define PAGE_SIZE	(1 << 12)
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

// buddy algorithms

// block is free
#define	UNUSED		0b00
// block isn't totally free, some sub blocks are allocated
#define DIRTY		0b01
// block is allocated
#define ALLOCATED	0b10
// block has all sub blocks allocated
#define UNAIVALABLE	0b11

#define GET_DBITS(map, i) \
	(((map[(i) >> 2] >> (2 * ((i) & 0x3))) & 0b11)

#define IS_USABLE(map, i) \
	GET_DBITS(map, i) < ALLOCATED)

#define IS_UNUSED(map, i) \
	GET_DBITS(map, i) == UNUSED)

#define IS_DIRTY(map, i) \
	GET_DBITS(map, i) == DIRTY)

#define IS_ALLOCATED(map, i) \
	GET_DBITS(map, i) == ALLOCATED)

#define SET(map, i, value) \
	map[(i) >> 2] = ((map[(i) >> 2] & ~(0b11 << (2 * ((i) & 0x3)))) \
	| (value << (2 * ((i) & 0x3))))

#define MAX_DEEP	19
#define MAP_LENGTH	(1 << 19)

#define GRANULARITY	2
#define GRANULARITY_NEG	2

u32			get_mem_area(u32 pages_req, u32 idx, u32 lvl, u8 *map);
u32			free_mem_area(u32 addr, u32 idx, u32 lvl, u8 *map);
int			mark_mem_area(u32 addr, u32 idx, u32 lvl, u32 cap,
				      u8 *map);

// kernel public function
void			*kmmap(u32 page_req);
int			kmunmap(void *addr);

// Kernel memory helpers
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
