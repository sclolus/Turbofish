
#include "memory.h"
#include "libft.h"

#define MAX_DEEP	11

#define BASE_PHYSICAL_ADDR	0x1000000
#define BASE_LINEAR_ADDRESS	0x800000
#define BMALLOC_DIRECTORY	2


#define PAGE_SIZE	(1 << 12)

#define MAP_FAILED	0xFFFFFFFF

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
			return MAP_FAILED;
		else
		{
			frame_map[index] = ALLOCATED;
			u32 segment = (index & g_page_mask[deep])
					* (1 << (MAX_DEEP - deep));
			u32 address = (index & g_page_mask[deep]) *
					(PAGE_SIZE << (MAX_DEEP - deep));
			return (paginate(
					BMALLOC_DIRECTORY + (segment >> 10),
					segment,
					1 << (MAX_DEEP - deep),
					address + BASE_PHYSICAL_ADDR));
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

		if (ret != MAP_FAILED) {
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

		if (ret != MAP_FAILED) {
			frame_map[index] = DIRTY;
			return ret;
		}
	}

	return MAP_FAILED;
}

void		*alloc_frames(u32 page_request)
{
	u32 ret;

	if (page_request > (1 << MAX_DEEP)
			|| page_request == 0
			|| frame_map[1] >= 2)
		return 0x0;

	ret = rec_alloc_frames(page_request, 1, 0);
	if (ret != MAP_FAILED)
		return (void *)(ret);
	return 0x0;
}

static int	rec_free_frames(u32 addr, u32 index, u32 deep)
{
	int ret;

	u32 ref_addr = BASE_LINEAR_ADDRESS + ((index & g_page_mask[deep])
			* (PAGE_SIZE << (MAX_DEEP - deep)));
	u32 sup_addr = ref_addr + ((PAGE_SIZE << MAX_DEEP) >> (1 + deep));

	if (deep > MAX_DEEP)
		return -1;

	if (addr == ref_addr && frame_map[index] == ALLOCATED)
	{
		frame_map[index] = UNUSED;

		u32 segment = (index & g_page_mask[deep])
				* (1 << (MAX_DEEP - deep));
		unpaginate(
				BMALLOC_DIRECTORY + (segment >> 10),
				segment,
				1 << (MAX_DEEP - deep));

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

	create_directory(BMALLOC_DIRECTORY);
	create_directory(BMALLOC_DIRECTORY + 1);
}

void		*bmalloc(size_t size)
{
	u32 frame_request = (size >> 12) + ((size & 0xFFF) ? 1 : 0);
	return alloc_frames(frame_request);
}

int		bfree(void *addr)
{
	return free_frames(addr);
}
