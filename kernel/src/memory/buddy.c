
#include "memory_manager.h"

#include "libft.h"

static u32 page_mask[21] = {
	0,
	0x1, 0x3, 0x7, 0xF,
	0x1F, 0x3F, 0x7F, 0xFF,
	0x1FF, 0x3FF, 0x7FF, 0xFFF,
	0x1FFF, 0x3FFF, 0x7FFF, 0xFFFF,
	0x1FFFF, 0x3FFFF, 0x7FFFF, 0xFFFFF};

/*
 * According to Buddy algorithm; this function return a free chunk witch
 * can contain the desired number of pages.
 * @initialization: for a search in all the map
 * get_mem_area(map, pages_req, 1, 0);
 * caution, never set 0 as index.
 * @return: Address and max number of pages in the chunk
 * on error: MAP_FAILLED
 */
struct mem_result	get_mem_area(u8 *map, u32 pages_req, u32 idx, u32 lvl)
{
	struct mem_result mem;

	if (lvl == MAX_LVL || pages_req
			> (u32)(GRANULARITY << (MAX_LVL - lvl - 1))) {
		if (IS_DIRTY(map, idx)) {
			return (struct mem_result){MAP_FAILED, 0};
		} else {
			if (idx > (MAP_LENGTH * GRANULARITY_NEG))
				eprintk("%s: ERROR idx, got %u\n",
						__func__, idx);
			SET(map, idx, ALLOCATED);
			u32 segment = (idx & page_mask[lvl])
					* (GRANULARITY << (MAX_LVL - lvl));
			mem.addr = (((segment >> 10) & 0x3FF) << 22)
				| ((segment & 0x3FF) << 12);
			mem.pages = GRANULARITY << (MAX_LVL - lvl);
			return mem;
		}
	}

	if (IS_USABLE(map, 2 * idx)) {
		mem = get_mem_area(map, pages_req, 2 * idx, lvl + 1);

		if ((!IS_USABLE(map, 2 * idx))
				&& (!IS_USABLE(map, 2 * idx + 1))) {
			SET(map, idx, UNAIVALABLE);
			return mem;
		}

		if (mem.addr != MAP_FAILED) {
			SET(map, idx, DIRTY);
			return mem;
		}
	}

	if (IS_USABLE(map, 2 * idx + 1)) {
		mem = get_mem_area(map, pages_req, 2 * idx + 1, lvl + 1);

		if ((!IS_USABLE(map, 2 * idx))
				&& (!IS_USABLE(map, 2 * idx + 1))) {
			SET(map, idx, UNAIVALABLE);
			return mem;
		}

		if (mem.addr != MAP_FAILED) {
			SET(map, idx, DIRTY);
			return mem;
		}
	}

	return (struct mem_result){MAP_FAILED, 0};
}

/*
 * This function return the container size like 1, 2, 4, 8 or 16 ...
 * The pointed chunk become free
 */
u32	free_mem_area(u8 *map, u32 addr, u32 idx, u32 lvl)
{
	int ret;

	if (lvl > MAX_LVL)
		return 0;

	u32 ref_addr = (idx & page_mask[lvl])
			* (u32)(PAGE_SIZE << (MAX_LVL - lvl)) * GRANULARITY;
	u32 sup_addr = ref_addr + ((u32)(PAGE_SIZE << (MAX_LVL - 1)) >>  lvl)
			* GRANULARITY;

	if (addr == ref_addr && IS_ALLOCATED(map, idx)) {
		SET(map, idx, UNUSED);
		return GRANULARITY << (MAX_LVL - lvl);
	} else if (addr < sup_addr) {
		ret = free_mem_area(map, addr, idx * 2, lvl + 1);
	} else {
		ret = free_mem_area(map, addr, idx * 2 + 1, lvl + 1);
	}

	if (ret != 0) {
		if (IS_UNUSED(map, idx * 2) &&
				IS_UNUSED(map, idx * 2 + 1))
			SET(map, idx, UNUSED);
		else
			SET(map, idx, DIRTY);
	}
	return ret;
}

/*
 * This function make a border between addressable area and not
 * physical memory is not unlimited
 * @parameters
 * 	map: memory map to utilize
 * 	limit_addr: area to make the border
 * 	index: current index on map in recursive scheme
 * 	level: current level on map in recursive scheme
 * @return
 *	integer: 0 on normal, -1 when error occurred
 *
 *	o---o-o			D---o-o    base become dirty
 *	 \   \			 \   \
 *	  \   o                   \   o      free zone
 *	   \         =>            \
 *	    o-o            ------   U-U -- unavailable limit
 *	     \                       \
 *	      o                       o      not allowed zone
 */
int	mark_area_limit(u8 *map, u32 limit_addr, u32 index, u32 level)
{
	u32 ref_addr;
	u32 sup_addr;
	int ret;

	/*
	 * error handling
	 */
	if (limit_addr == 0 || (limit_addr & PAGE_MASK))
		return -1;

	/*
	 * stop condition
	 */
	if (level == MAX_LVL) {
		SET(map, index, UNAIVALABLE);
		return 0;
	}

	/*
	 * recursive descent
	 * ref_addr is the current address location
	 * sup_addr is the upper address of right buddy
	 */
	ref_addr = (index & page_mask[level])
			* (u32)(PAGE_SIZE << (MAX_LVL - level)) * GRANULARITY;
	sup_addr = ref_addr + ((u32)(PAGE_SIZE << (MAX_LVL - 1)) >>  level)
			* GRANULARITY;

	if (limit_addr < sup_addr)
		ret = mark_area_limit(map, limit_addr, index * 2, level + 1);
	else
		ret = mark_area_limit(
				map, limit_addr, index * 2 + 1, level + 1);

	/*
	 * conclusion, while ascent
	 * mark all address equal or greater as unavailable
	 * mark all above address on the way as dirty
	 */
	if (ref_addr == limit_addr)
		SET(map, index, UNAIVALABLE);
	else
		SET(map, index, DIRTY);

	if (sup_addr > limit_addr)
		SET(map, 2 * index + 1, UNAIVALABLE);

	return ret;
}

/*
 * This function directly mark a chunk as ALLOCATED in the desired address
 * on the desired deep (cap). It's necessary to understand exactly how the
 * buddy system work before using this this function.
 * @return: -1 on error, the cap on success
 */
int	mark_mem_area(u8 *map, u32 addr, u32 idx, u32 lvl, u32 cap)
{
	int ret;

	if (lvl > MAX_LVL)
		return -1;

	if (!IS_USABLE(map, idx))
		return -1;

	u32 ref_addr = (idx & page_mask[lvl])
			* (u32)(PAGE_SIZE << (MAX_LVL - lvl)) * GRANULARITY;
	u32 sup_addr = ref_addr + ((u32)(PAGE_SIZE << (MAX_LVL - 1)) >> lvl)
			* GRANULARITY;

	if (lvl == cap) {
		if (addr == ref_addr && IS_UNUSED(map, idx)) {
			SET(map, idx, ALLOCATED);
			return lvl;
		}
		return -1;
	}

	if (addr < sup_addr)
		ret = mark_mem_area(map, addr, idx * 2, lvl + 1, cap);
	else
		ret = mark_mem_area(map, addr, idx * 2 + 1, lvl + 1, cap);

	if (ret != -1) {
		if ((!IS_USABLE(map, 2 * idx))
				&& (!IS_USABLE(map, 2 * idx + 1))) {
			SET(map, idx, UNAIVALABLE);
			return ret;
		}

		SET(map, idx, DIRTY);
		return ret;
	}
	return ret;
}

/*
 * Very difficult to understand, this function mark multiples deeper chunks
 * and associate mapping for conversion from virt_addr to phy_addr. For
 * example, a VMALLOC call may associate multiple and different physical area.
 * @return: -1 on error, 0 on success
 */
int	mem_multiple_area(
	u8 *map,
	u32 *pages_req,
	u32 idx,
	u32 lvl,
	u32 *virt_addr,
	int (*map_fn)(u32 virt_addr, u32 page_req, u32 phy_addr,
			enum mem_space space))
{
	u32	block_size;
	int	ret = 0;

	block_size = GRANULARITY << (MAX_LVL - lvl);
	if (*pages_req >= block_size && IS_UNUSED(map, idx)) {
		u32 ref_addr = (idx & page_mask[lvl])
				* (u32)(PAGE_SIZE << (MAX_LVL - lvl))
				* GRANULARITY;

#ifdef DEBUG_VALLOC
		printk("virt %p phy %p for %u,",
				*virt_addr,
				ref_addr,
				block_size * PAGE_SIZE);
#endif
		ret = map_fn(*virt_addr, block_size, ref_addr, kernel_space);
		if (ret == -1)
			return -1;

		SET(map, idx, ALLOCATED);
		*pages_req -= block_size;
		*virt_addr += PAGE_SIZE * block_size;
		return ret;
	}

	if (IS_USABLE(map, 2 * idx)) {
		ret = mem_multiple_area(
				map,
				pages_req,
				2 * idx,
				lvl + 1,
				virt_addr,
				map_fn);
		if (ret == -1)
			return -1;

		if ((!IS_USABLE(map, 2 * idx))
				&& (!IS_USABLE(map, 2 * idx + 1)))
			SET(map, idx, UNAIVALABLE);
		else
			SET(map, idx, DIRTY);

		if (*pages_req == 0)
			return ret;
	}

	if (IS_USABLE(map, 2 * idx + 1)) {
		ret = mem_multiple_area(
				map,
				pages_req,
				2 * idx + 1,
				lvl + 1,
				virt_addr,
				map_fn);
		if (ret == -1)
			return -1;

		if ((!IS_USABLE(map, 2 * idx))
				&& (!IS_USABLE(map, 2 * idx + 1)))
			SET(map, idx, UNAIVALABLE);
		else
			SET(map, idx, DIRTY);

		if (*pages_req == 0)
			return ret;
	}
	return ret;
}
