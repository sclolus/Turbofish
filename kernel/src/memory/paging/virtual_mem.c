
#include "memory_manager.h"
#include "libft.h"

#define VIRT_MAP_LOCATION	0x300000

static u8 *virt_map;

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
			if (!IS_USABLE(virt_map, 4))
				return (void *)MAP_FAILED;
			virt_addr = get_mem_area(page_request, 4, 2, virt_map);
			break;
		case user_space:
			if (page_request > (1 << SHL_LIMIT_1GO_BLOCK))
				return (void *)MAP_FAILED;
			if (!IS_USABLE(virt_map, 5))
				virt_addr = MAP_FAILED;
			else
				virt_addr = get_mem_area(
						page_request,
						5,
						2,
						virt_map);
			if (virt_addr == MAP_FAILED)
			{
				if (page_request > (1 << SHL_LIMIT_2GO_BLOCK))
					return (void *)MAP_FAILED;
				if (!IS_USABLE(virt_map, 3))
					return (void *)MAP_FAILED;
				virt_addr
				= get_mem_area(page_request, 3, 1, virt_map);
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
			ret = free_mem_area((u32)addr, 4, 2, virt_map);
			break;
		case user_space:
			ret = free_mem_area((u32)addr, 5, 2, virt_map);
			if (ret == 0)
				ret = free_mem_area((u32)addr, 3, 1, virt_map);
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
	ft_memset(virt_map, 0, MAP_LENGTH);
}
