
#include "buddy.h"

#include "libft.h"

static u8 kernel_virtual_buddy[1024 * 512] __attribute__((aligned(32))) = {0};

/*
 * Buddy IDX Organisation
 * 4g   2g      1g            512m           256m      128m     64m
 * 1   2,3   4,5,6,7   8,9,10,11,12,13,14,15   16..... 32....   64
 *
 * Placed in High Half Memory 0xC0000000
 * First virtual G byte: Reserved for Kernel Usage Only
 * |       |        |        |        |        |        |        |        |
 * 0       128      256               512                                 1024
 *         |        |                                                     |
 *  XXXXXX |VHEAP-->|KHEAP ---------------------------------------------->|
 */
#define SHL_LIMIT_128M_BLOCK   17
#define SHL_LIMIT_256M_BLOCK   18
#define SHL_LIMIT_512M_BLOCK   19
#define SHL_LIMIT_1G_BLOCK     20
#define SHL_LIMIT_2G_BLOCK     21

/*
 * This chunk is reserved for kernel CS and other important stuff
 * 56 is the first chunk of 128mo in high half 0xc0000000
 */
#define RESERVED_IDX           56
#define RESERVED_DEEP          5

/*
 * 29 is index of the second 256mo chunk in high half 0xc0000000
 * 15 is the index of the last 512mo chunk
 */
#define KHEAP_FIRST_IDX        29
#define KHEAP_FIRST_DEEP       4
#define KHEAP_SECOND_IDX       15
#define KHEAP_SECOND_DEEP      3

/*
 * 57 is index of the second 128mo chunk in high half 0xc0000000
 */
#define VHEAP_IDX              57  // 0xc8000000
#define VHEAP_DEEP             5

/*
 * 2 is the index of the first 2go chunk
 * 6 is the index of the third 1go chunk
 */
#define USER_FIRST_IDX         2
#define USER_FIRST_DEEP        1
#define USER_SECOND_IDX        6
#define USER_SECOND_DEEP       2


#define FIRST_MO_IDX           4096
#define FIRST_MO_DEEP          12

/*
 * XXX VIRTUAL MEMORY ORGANISATION XXX
 *             ^ ------------------------------
 *             |
 *             | 1go Block KERNEL SPACE
 *          7  |-------------------------------
 *         /   |  USER_SPACE First Virtual 3GO
 *        /    |  0x0 -> 0xBFFFFFFF
 *       3--6  |
 *      /      |  3Go block
 *     /       |
 *    /     5  |
 *   /     /   |
 *  /     /    |
 * 1-----2--4 -v---------------------------------
 * Index number
 *                         high_half 0xC0000000   deep
 * 1 => 4go                       X                0
 * 2 --> 3 => 2go                 X                1
 * 4 --> 7 => 1go                 7                2
 * 8 --> 15 => 512mo              14               3
 * 16 --> 31 => 256mo             28               4
 * 32 --> 63 => 128mo             56               5
 *
 * 64 --> 127 => 64mo                              6
 * 128 --> 255 => 32mo                             7
 * 256 --> 511 => 16mo                             8
 * 512 --> 1023 => 8mo                             9
 * 1024 --> 2047 => 4mo                           10
 * 2048 --> 4095 => 2mo                           11
 * 4096 --> 8191 => 1mo                           12
 */
u32	get_pages(u32 page_request, enum mem_type type)
{
	u32 addr;

	if (page_request == 0)
		return MAP_FAILED;

	addr = MAP_FAILED;

	switch (type) {
	case first_mo:
		addr = get_mem_area(
				kernel_virtual_buddy,
				256,
				FIRST_MO_IDX,
				FIRST_MO_DEEP);
		break;
	case reserved:
		if (page_request <= (1 << SHL_LIMIT_128M_BLOCK) &&
				IS_USABLE(kernel_virtual_buddy, RESERVED_IDX))
			addr = get_mem_area(
					kernel_virtual_buddy,
					page_request,
					RESERVED_IDX,
					RESERVED_DEEP);
		if (addr != MAP_FAILED)
			break;
		break;

	case kheap:
		if (page_request <= (1 << SHL_LIMIT_256M_BLOCK) &&
				IS_USABLE(kernel_virtual_buddy, KHEAP_FIRST_IDX))
			addr = get_mem_area(
					kernel_virtual_buddy,
					page_request,
					KHEAP_FIRST_IDX,
					KHEAP_FIRST_DEEP);
		if (addr != MAP_FAILED)
			break;

		if (page_request <= (1 << SHL_LIMIT_512M_BLOCK) &&
				IS_USABLE(kernel_virtual_buddy, KHEAP_SECOND_IDX))
			addr = get_mem_area(
					kernel_virtual_buddy,
					page_request,
					KHEAP_SECOND_IDX,
					KHEAP_SECOND_DEEP);
		if (addr != MAP_FAILED)
			break;
		break;

	case vheap:
		if (page_request <= (1 << SHL_LIMIT_128M_BLOCK) &&
				IS_USABLE(kernel_virtual_buddy, VHEAP_IDX))
			addr = get_mem_area(
					kernel_virtual_buddy,
					page_request,
					VHEAP_IDX,
					VHEAP_DEEP);
		if (addr != MAP_FAILED)
			break;
		break;

	case usermem:
		if (page_request <= (1 << SHL_LIMIT_1G_BLOCK) &&
				IS_USABLE(kernel_virtual_buddy, USER_FIRST_IDX))
			addr = get_mem_area(
					kernel_virtual_buddy,
					page_request,
					USER_FIRST_IDX,
					USER_FIRST_DEEP);
		if (addr != MAP_FAILED)
			break;

		if (page_request <= (1 << SHL_LIMIT_2G_BLOCK) &&
				IS_USABLE(kernel_virtual_buddy, USER_SECOND_IDX))
			addr = get_mem_area(
					kernel_virtual_buddy,
					page_request,
					USER_SECOND_IDX,
					USER_SECOND_DEEP);
		if (addr != MAP_FAILED)
			break;
		break;

	default:
		eprintk("%s: Unexpected default status\n", __func__);
		break;
	}
	return addr;
}

u32	free_pages(void *addr, enum mem_type type)
{
	int ret;

	switch (type) {
	case first_mo:
	case reserved:
		eprintk("%s: Cannot FREE reserved or first mo pages\n", __func__);
		ret = 0;
		break;
	case kheap:
		ret = free_mem_area(
				kernel_virtual_buddy,
				(u32)addr,
				KHEAP_FIRST_IDX,
				KHEAP_FIRST_DEEP);
		if (ret != 0)
			break;

		ret = free_mem_area(
				kernel_virtual_buddy,
				(u32)addr,
				KHEAP_SECOND_IDX,
				KHEAP_SECOND_DEEP);
		break;
	case vheap:
		ret = free_mem_area(
				kernel_virtual_buddy,
				(u32)addr,
				VHEAP_IDX,
				VHEAP_DEEP);
		break;
	case usermem:
		ret = free_mem_area(
				kernel_virtual_buddy,
				(u32)addr,
				USER_FIRST_IDX,
				USER_FIRST_DEEP);
		if (ret != 0)
			break;

		ret = free_mem_area(
				kernel_virtual_buddy,
				(u32)addr,
				USER_SECOND_IDX,
				USER_SECOND_DEEP);
		break;
	default:
		eprintk("%s: Unexpected default status\n", __func__);
		ret = 0;
		break;
	}
	return ret;
}

int	mark_virtual_area(void *virt_addr, u32 page_request)
{
	return mark_area(kernel_virtual_buddy, virt_addr, page_request);
}

