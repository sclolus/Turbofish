
#ifdef DIRECT_VALLOC
Function were present is memory/physical_mem.c
int			write_multiple_physical_addr(
			u32 page_request,
			void *virt_addr,
			int (*map)(u32 virt_addr, u32 page_req, u32 phy_addr,
					enum mem_space space));
/*
 * This function is called like that
 * int ret = write_multiple_physical_addr(
 *				page_request,
 *				(void *)virt_addr,
 *				&map_address);
 */
int		write_multiple_physical_addr(
		u32 page_request,
		void *virt_addr,
		int (*map_fn)(u32 virt_addr, u32 page_req, u32 phy_addr,
				enum mem_space space))
{
	u32 ptr;

	ptr = (u32)virt_addr;
#ifdef DEBUG_VALLOC
	printk("{red}VIRT: %p for %u pages -> {eoc}", virt_addr, page_request);
#endif
	return mem_multiple_area(phy_map, &page_request, 1, 0, &ptr, map_fn);
}

Function were present is memory/buddy.c
int			mem_multiple_area(
			u8 *map,
			u32 *pages_req,
			u32 idx,
			u32 lvl,
			u32 *virt_addr,
			int (*map_fn)(u32 virt_addr, u32 page_req,
					u32 phy_addr, enum mem_space space));

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
#endif
