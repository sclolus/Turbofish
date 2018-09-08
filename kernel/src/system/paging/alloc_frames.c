
#include "paging.h"

#include "libft.h"

#define MAX_DEEP	11

#define BASE_ADDR	0x0
#define PAGE_SIZE	(1 << 12)

#define NO_ALLOC	0xFFFFFFFF

static u32 g_page_mask[MAX_DEEP + 1] =
	{0, 0x1, 0x3, 0x7, 0xF, 0x1F, 0x3F, 0x7F, 0xFF, 0x1FF, 0x3FF, 0x7FF};

#define	UNUSED		0x0	// block is free
#define DIRTY		0x1	// block is not totally free, some sub blocks are allocated
#define ALLOCATED	0x2	// block is allocated
#define UNAIVALABLE	0x3	// block has all sub blocks allocated

static u8 frame_map[1 << (MAX_DEEP + 1)];

static u32	rec_alloc_frames(u32 page_request, u32 index, u32 deep)
{
	u32	ret;

	if (deep == MAX_DEEP
			|| page_request > (u32)(1 << (MAX_DEEP - deep - 1)))
	{
		if (frame_map[index] == DIRTY)
			return NO_ALLOC;
		else
		{
			frame_map[index] = ALLOCATED;
			return (index & g_page_mask[deep]) *
					(PAGE_SIZE << (MAX_DEEP - deep));
		}
	}

	if (frame_map[2 * index] <= 0x1)
	{
		ret = rec_alloc_frames(page_request, 2 * index, deep + 1);

		if (frame_map[2 * index] >= 0x2 &&
				frame_map[2 * index + 1] >= 0x2)
		{
			frame_map[index] = UNAIVALABLE;
			return ret;
		}

		if (ret != NO_ALLOC) {
			frame_map[index] = DIRTY;
			return ret;
		}
	}

	if (frame_map[2 * index + 1] <= 0x1)
	{
		ret = rec_alloc_frames(page_request, 2 * index + 1, deep + 1);

		if (frame_map[2 * index] >= 0x2 &&
				frame_map[2 * index + 1] >= 0x2)
		{
			frame_map[index] = UNAIVALABLE;
			return ret;
		}

		if (ret != NO_ALLOC) {
			frame_map[index] = DIRTY;
			return ret;
		}
	}

	return NO_ALLOC;
}

void		*alloc_frames(u32 page_request)
{
	u32 ret;

	if (page_request > (1 << MAX_DEEP)
			|| page_request == 0
			|| frame_map[1] >= 2)
		return 0x0;

	ret = rec_alloc_frames(page_request, 1, 0);
	if (ret != NO_ALLOC)
		return (void *)(ret + BASE_ADDR);
	return 0x0;
}

static int	rec_free_frames(u32 addr, u32 index, u32 deep)
{
	int ret;
	u32 ref_addr = ((index & g_page_mask[deep])
			* (PAGE_SIZE << (MAX_DEEP - deep)));
	u32 sup_addr = ref_addr + ((PAGE_SIZE << MAX_DEEP) >> (1 + deep));

	if (deep > MAX_DEEP)
		return -1;

	if (addr == ref_addr && frame_map[index] == ALLOCATED)
	{
		frame_map[index] = UNUSED;
		return 0;
	}
	else if (addr < sup_addr)
		ret = rec_free_frames(addr, index * 2, deep + 1);
	else
		ret = rec_free_frames(addr, index * 2 + 1, deep + 1);

	if (ret != -1)
	{
		if (frame_map[index * 2] == UNUSED &&
				frame_map[index * 2 + 1] == UNUSED)
			frame_map[index] = UNUSED;
	}
	return ret;
}

int		free_frames(void *addr)
{
	return rec_free_frames((u32)addr, 1, 0);
}

u32		count_frames(void)
{
	int count = 0;

	for (int i = 0; i < 1 << (MAX_DEEP + 1); i++)
	{
		if (frame_map[i] == ALLOCATED)
			count++;
	}
	return count;
}

void		init_frames(void)
{
	ft_memset(&frame_map, 1 << (MAX_DEEP + 1), UNUSED);
}
