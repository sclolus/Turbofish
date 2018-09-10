
#include "memory_manager.h"
#include "libft.h"

// 2^12 = 4096 * 16ko = 64mo
// 2^16 = 1go
// 2^28 = 4go
#define MAX_DEEP		18
#define PAGE_SIZE		(1 << 12)
#define VIRT_MAP_LOCATION	0x300000
#define VIRT_MAP_LENGTH		(1 << 19)

static u32 g_page_mask[MAX_DEEP + 1] = {
	0,
	0x1, 0x3, 0x7, 0xF,
	0x1F, 0x3F, 0x7F, 0xFF,
	0x1FF, 0x3FF, 0x7FF, 0xFFF,
	0x1FFF, 0x3FFF, 0x7FFF, 0xFFFF,
	0x1FFFF, 0x3FFFF};

// block is free
#define	UNUSED		0x1
// block isn't totally free, some sub blocks are allocated
#define DIRTY		0x2
// block is allocated
#define ALLOCATED	0x4
// block has all sub blocks allocated
#define UNAIVALABLE	0x8

#define SHL_LIMIT_1GO_BLOCK	20
#define SHL_LIMIT_2GO_BLOCK	21

/*
 * XXX VIRTUAL MEMORY ORGANISATION XXX
 *             ^ ------------------------------
 *             |
 *             | 2 Go block
 *          7  |
 *         /   |  USER_SPACE Last Virtual 3GO
 *        /    |  0x40000000 -> 0xFFFFFFFF
 *       3--6  |
 *      /      | 1Go block
 *     /       |
 *    /     5  v -------- ^ ---------------------
 *   /     /              | 0x0 -> 0x3FFFFFFF
 *  /     /   KERN_SPACE  | First Virtual 1GO
 * 1-----2--4 ----------- v ---------------------
 * Index number
 */

static u8 *virt_map;

#define IS_USABLE(i)	((virt_map[i] & UNUSED) || (virt_map[i] & DIRTY))


static u32	rec_get_frames(u32 page_request, u32 index, u32 deep)
{
	u32	ret;

	if (deep == MAX_DEEP
			|| page_request > (u32)(4 << (MAX_DEEP - deep - 1)))
	{
		if (virt_map[index] & DIRTY)
			return MAP_FAILED;
		else
		{
			if (index > VIRT_MAP_LENGTH)
				eprintk("%s: ERROR index, got %u\n",
						__func__, index);
			virt_map[index] = ALLOCATED;
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
			virt_map[index] = UNAIVALABLE;
			return ret;
		}

		if (ret != MAP_FAILED) {
			virt_map[index] = DIRTY;
			return ret;
		}
	}

	if (IS_USABLE(2 * index + 1))
	{
		ret = rec_get_frames(page_request, 2 * index + 1, deep + 1);

		if ((!IS_USABLE(2 * index)) && (!IS_USABLE(2 * index + 1)))
		{
			virt_map[index] = UNAIVALABLE;
			return ret;
		}

		if (ret != MAP_FAILED) {
			virt_map[index] = DIRTY;
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

	if (addr == ref_addr && (virt_map[index] & ALLOCATED))
	{
		virt_map[index] = UNUSED;
		return 4 << (MAX_DEEP - deep);
	}
	else if (addr < sup_addr)
		ret = rec_free_pages(addr, index * 2, deep + 1);
	else
		ret = rec_free_pages(addr, index * 2 + 1, deep + 1);

	if (ret != 0)
	{
		if ((virt_map[index * 2] & UNUSED) &&
				(virt_map[index * 2 + 1] & UNUSED))
			virt_map[index] = UNUSED;
	}
	return ret;
}

void		*get_pages(u32 page_request, enum mem_space space)
{
	u32 virt_addr;

	if (page_request == 0)
		return (void *)MAP_FAILED;

	switch (space)
	{
		case kernel_space:
			if (page_request > (1 << SHL_LIMIT_1GO_BLOCK))
				return (void *)MAP_FAILED;
			if (!IS_USABLE(4))
				return (void *)MAP_FAILED;
			virt_addr = rec_get_frames(page_request, 4, 2);
			break;
		case user_space:
			if (page_request > (1 << SHL_LIMIT_1GO_BLOCK))
				return (void *)MAP_FAILED;
			if (!IS_USABLE(5))
				virt_addr = MAP_FAILED;
			else
				virt_addr = rec_get_frames(
						page_request,
						5,
						2);
			if (virt_addr == MAP_FAILED)
			{
				if (page_request > (1 << SHL_LIMIT_2GO_BLOCK))
					return (void *)MAP_FAILED;
				if (!IS_USABLE(3))
					return (void *)MAP_FAILED;
				virt_addr
				= rec_get_frames(page_request, 3, 1);
			}
			break;
		default:
			eprintk("%s: Unexpected default status\n");
			break;
	}
	return (void *)(virt_addr);
}

u32		free_pages(void *addr, enum mem_space space)
{
	int ret;

	switch (space)
	{
		case kernel_space:
			ret = rec_free_pages((u32)addr, 4, 2);
			break;
		case user_space:
			ret = rec_free_pages((u32)addr, 5, 2);
			if (ret == 0)
				ret = rec_free_pages((u32)addr, 3, 1);
			break;
		default:
			eprintk("%s: Unexpected default status\n");
			ret = 0;
			break;
	}
	return ret;
}

void		init_virtual_map(void)
{
	virt_map = (u8 *)VIRT_MAP_LOCATION;
	ft_memset(virt_map, UNUSED, VIRT_MAP_LENGTH);
}
