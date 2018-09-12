
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

struct mem_result {
	u32	addr;
	size_t	pages;
};

struct mem_result	get_pages(u32 page_request, enum mem_space space);
u32			free_pages(void *addr, enum mem_space space);

// physical map internal functions
void			init_physical_map(void);
int			mark_physical_area(void *addr, u32 page_request);
void			*get_physical_addr(u32 page_request);
int			drop_physical_addr(void *addr);

int			write_multiple_physical_addr(
			u32 page_request,
			void *virt_addr,
			int (*map)(u32 virt_addr, u32 page_req, u32 phy_addr,
					enum mem_space space));

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

#define MAX_LVL		20
#define MAP_LENGTH	(1 << 19)

/*
 * granularity is the number of 4ko pages for the more tiny area
 * his NEG value is the opposite. (g4 n1, g2 n2, g1, n4)
 */

#define GRANULARITY	1
#define GRANULARITY_NEG	4

struct mem_result	get_mem_area(u8 *map, u32 pages_req, u32 idx, u32 lvl);
u32			free_mem_area(u8 *map, u32 addr, u32 idx, u32 lvl);
int			mark_mem_area(u8 *map, u32 addr, u32 idx, u32 lvl,
				      u32 cap);
int			mem_multiple_area(
			u8 *map,
			u32 *pages_req,
			u32 idx,
			u32 lvl,
			u32 *virt_addr,
			int (*map_fn)(u32 virt_addr, u32 page_req,
					u32 phy_addr, enum mem_space space));

// kernel public function
void			*kmmap(size_t size);
void			*vmmap(size_t size);

int			kmunmap(void *virt_addr);
int			vmunmap(void *virt_addr);

// Kernel K-Family memory helpers
void			*kmalloc(size_t size);
void			kfree(void *ptr);
void			*kcalloc(size_t count, size_t size);
void			*krealloc(void *ptr, size_t size);
void			*kreallocf(void *ptr, size_t size);
void			*kreallocarray(void *ptr, size_t nmemb, size_t size);
void			*kvalloc(size_t size);
void			kshow_alloc_mem(void);
void			kshow_alloc_mem_ex(void);

// Kernel V-Family memory helpers
void			*valloc(size_t size);
int			vfree(void *ptr);
size_t			vsize(void *ptr);

#endif
