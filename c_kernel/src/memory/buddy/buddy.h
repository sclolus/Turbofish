
#ifndef __BUDDY_H__
# define __BUDDY_H__

#include "i386_type.h"

#define PAGE_SIZE	(1 << 12)
#define PAGE_MASK	0xFFF
#define MAP_FAILED	0xFFFFFFFF

enum mem_type {
	reserved = 0,
	kheap,
	vheap,
	usermem,
	first_mo,
};

enum mem_space {
	kernel_space = 0,
	user_space
};

/*
 * buddy algorithms macros
 */

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

/*
 * Virtual Map internal functions
 */
u32			get_pages(u32 page_request, enum mem_type type);
u32			free_pages(void *addr, enum mem_type type);
int			mark_virtual_area(void *virt_addr, u32 page_request);

/*
 * Physical map internal functions
 */
void			init_physical_map(void *limit_phy_addr);
int			mark_physical_area(void *addr, u32 page_request);
void			*get_physical_addr(u32 page_request);
int			drop_physical_addr(void *addr);

/*
 * Common functions
 */
int			mark_area(u8 *buddy, void *addr, u32 page_request);

/*
 * Buddy specialised functions
 */
u32			get_mem_area(u8 *map, u32 pages_req, u32 idx, u32 lvl);
u32			free_mem_area(u8 *map, u32 addr, u32 idx, u32 lvl);
int			mark_mem_area(u8 *map, u32 addr, u32 idx, u32 lvl,
				      u32 cap);
int			mark_area_limit(
			u8 *map,
			u32 limit_addr,
			u32 idx,
			u32 lvl);

#endif
