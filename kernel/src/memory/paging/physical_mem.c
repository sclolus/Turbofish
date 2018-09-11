
#include "memory_manager.h"
#include "libft.h"

#define MAX_DEEP		18
#define PAGE_SIZE		(1 << 12)
#define PHY_MAP_LOCATION	0x380000
#define PHY_MAP_LENGTH		(1 << 19)

static u32 g_page_mask[MAX_DEEP + 1] = {
	0,
	0x1, 0x3, 0x7, 0xF,
	0x1F, 0x3F, 0x7F, 0xFF,
	0x1FF, 0x3FF, 0x7FF, 0xFFF,
	0x1FFF, 0x3FFF, 0x7FFF, 0xFFFF,
	0x1FFFF, 0x3FFFF};

// block is free
#define	UNUSED			0x1
// block isn't totally free, some sub blocks are allocated
#define DIRTY			0x2
// block is allocated
#define ALLOCATED		0x4
// block has all sub blocks allocated
#define UNAIVALABLE		0x8

static u8 *phy_map;

#define IS_USABLE(i)	((phy_map[i] & UNUSED) || (phy_map[i] & DIRTY))


static u32	rec_get_frames(u32 page_request, u32 index, u32 deep)
{
	u32	ret;

	if (deep == MAX_DEEP
			|| page_request > (u32)(4 << (MAX_DEEP - deep - 1)))
	{
		if (phy_map[index] & DIRTY)
			return MAP_FAILED;
		else
		{
			if (index > PHY_MAP_LENGTH)
				eprintk("%s: ERROR index, got %u\n",
						__func__, index);
			phy_map[index] = ALLOCATED;
			u32 segment = (index & g_page_mask[deep])
					* (4 << (MAX_DEEP - deep));
			return (((segment >> 10) & 0x3FF) << 22)
					| ((segment & 0x3FF) << 12);
		}
	}

	if (IS_USABLE(2 * index))
	{
		ret = rec_get_frames(page_request, 2 * index, deep + 1);

		if ((!IS_USABLE(2 * index)) && (!IS_USABLE(2 * index + 1)))
		{
			phy_map[index] = UNAIVALABLE;
			return ret;
		}

		if (ret != MAP_FAILED) {
			phy_map[index] = DIRTY;
			return ret;
		}
	}

	if (IS_USABLE(2 * index + 1))
	{
		ret = rec_get_frames(page_request, 2 * index + 1, deep + 1);

		if ((!IS_USABLE(2 * index)) && (!IS_USABLE(2 * index + 1)))
		{
			phy_map[index] = UNAIVALABLE;
			return ret;
		}

		if (ret != MAP_FAILED) {
			phy_map[index] = DIRTY;
			return ret;
		}
	}

	return MAP_FAILED;
}

static u32	rec_free_pages(u32 addr, u32 index, u32 deep)
{
	int ret;

	if (deep > MAX_DEEP)
		return 0;

	u32 ref_addr = (index & g_page_mask[deep])
			* (PAGE_SIZE << (MAX_DEEP - deep)) * 4;
	u32 sup_addr = ref_addr + ((PAGE_SIZE << (MAX_DEEP - 1)) >>  deep) * 4;

	if (addr == ref_addr && (phy_map[index] & ALLOCATED))
	{
		phy_map[index] = UNUSED;
		return 4 << (MAX_DEEP - deep);
	}
	else if (addr < sup_addr)
		ret = rec_free_pages(addr, index * 2, deep + 1);
	else
		ret = rec_free_pages(addr, index * 2 + 1, deep + 1);

	if (ret != 0)
	{
		if ((phy_map[index * 2] & UNUSED) &&
				(phy_map[index * 2 + 1] & UNUSED))
			phy_map[index] = UNUSED;
	}
	return ret;
}

/*
 * Addressing of all 4go memory
 */
void		*get_physical_addr(u32 page_request)
{
	u32 phy_addr;

	if (page_request == 0)
		return (void *)MAP_FAILED;

	if (!IS_USABLE(1))
		return (void *)MAP_FAILED;

	phy_addr = rec_get_frames(page_request, 1, 0);

	return (void *)(phy_addr);
}

int		drop_physical_addr(void *addr)
{
	return rec_free_pages((u32)addr, 1, 0);
}

static size_t	count_bits(u32 ref)
{
	size_t count = 0;

	while (ref)
	{
		count++;
		ref >>= 1;
	}
	return count;
}

static int	mark_page(u32 addr, u32 index, u32 deep, u32 cap)
{
	int ret;

	if (deep > MAX_DEEP)
		return -1;

	u32 ref_addr = (index & g_page_mask[deep])
			* (PAGE_SIZE << (MAX_DEEP - deep)) * 4;
	u32 sup_addr = ref_addr + ((PAGE_SIZE << (MAX_DEEP - 1)) >>  deep) * 4;

	if (deep == cap)
	{
		if (addr == ref_addr && (phy_map[index] & UNUSED))
		{
			phy_map[index] = ALLOCATED;
			return 0;
		}
		return -1;
	}

	if (addr < sup_addr)
		ret = mark_page(addr, index * 2, deep + 1, cap);
	else
		ret = mark_page(addr, index * 2 + 1, deep + 1, cap);

	if (ret != -1)
	{
		if ((!IS_USABLE(2 * index)) && (!IS_USABLE(2 * index + 1)))
		{
			phy_map[index] = UNAIVALABLE;
			return ret;
		}

		phy_map[index] = DIRTY;
		return ret;
	}
	return ret;
}

int		mark_physical_area(void *addr, u32 page_request)
{
	size_t	bitlen;
	u32	deep;

	if (page_request == 0)
		return -1;

	bitlen = count_bits(page_request);
	if (bitlen <= 2)
		deep = MAX_DEEP;
	else
	{
		deep = MAX_DEEP - bitlen + 2;
		if (page_request == (u32)((1 << (bitlen - 1))))
			deep++;
	}

	return mark_page((u32)addr, 1, 0, deep);
}

void		init_physical_map(void)
{
	phy_map = (u8 *)PHY_MAP_LOCATION;
	ft_memset(phy_map, UNUSED, PHY_MAP_LENGTH);
}
