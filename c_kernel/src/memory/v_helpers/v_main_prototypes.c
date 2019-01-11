
#include "../memory_manager.h"
#include "dynamic_allocator.h"

#include "libft.h"

struct v_record {
	void *next;
	void *addr;
	size_t size_max;
	size_t current_size;
};

struct valloc_ctx {
	struct v_record *record;
};

static struct valloc_ctx *valloc_ctx = NULL;

static int		init_valloc(void)
{
	valloc_ctx = (struct valloc_ctx *)kmalloc(sizeof(struct valloc_ctx));
	if (valloc_ctx == NULL)
		return -1;
	valloc_ctx->record = NULL;
	return 0;
}

void			*valloc(size_t size)
{
	struct v_record *record;
	struct v_record *current;
	struct v_record *prev;

	if (valloc_ctx == NULL && init_valloc() == -1)
		return NULL;

	record = (struct v_record *)kmalloc(sizeof(struct v_record));
	if (record == NULL)
		return NULL;

	/*
	 * Get a virtual area
	 */
	record->addr = vmmap(size);
	if (record->addr == (void *)MAP_FAILED) {
		kfree(record);
		return NULL;
	};
	record->current_size = 0;
	record->size_max = size;

	current = valloc_ctx->record;
	prev = NULL;
	while (current && (record->addr < current->addr)) {
		prev = current;
		current = current->next;
	}

	if (prev == NULL) {
		record->next = current;
		valloc_ctx->record = record;
	} else {
		record->next = current;
		prev->next = record;
	}
	return record->addr;
}

int			vfree(void *addr)
{
	struct v_record	*record;
	struct v_record	*prev;
	int		ret;

	if (valloc_ctx == NULL && init_valloc() == -1)
		return NULL;

	record = valloc_ctx->record;
	prev = NULL;
	while (record != NULL) {
		if (addr == record->addr) {
			if (prev != NULL)
				prev->next = record->next;
			else
				valloc_ctx->record = record->next;
			/*
			* Now, free physical and virtual area
			*/
			ret = vmunmap(record->addr, record->current_size);
			kfree(record);
			return ret;
		}
		prev = record;
		record = record->next;
	}
	return -1;
}

size_t			vsize(void *addr)
{
	struct v_record *record;

	if (valloc_ctx == NULL && init_valloc() == -1)
		return NULL;

	record = valloc_ctx->record;
	while (record) {
		if (addr == record->addr)
			return record->current_size;
		record = record->next;
	}
	return 0;
}

int			v_assign_phy_area(u32 fault_addr)
{
	struct v_record	*record;
	void		*phy_addr;
	size_t		tmp_size;

	if (valloc_ctx == NULL && init_valloc() == -1)
		return NULL;

	record = valloc_ctx->record;
	while (record) {
		if (fault_addr >= (u32)record->addr
				&& fault_addr < (u32)record->addr +
				record->size_max) {
			/*
			 * Set the size of the allocated chunk, be careful,
			 * a write operation can be done after the first 4096o
			 */
			tmp_size = fault_addr - (u32)record->addr;
			tmp_size = PAGE_SIZE * ((tmp_size >> 12) + 1);
			if (tmp_size > record->current_size)
				record->current_size = tmp_size;

			/*
			 * Now, find a physical chunk, assign it and refresh TLB
			 */
			phy_addr = get_physical_addr(1);
			if ((u32)phy_addr == MAP_FAILED) {
				eprintk("%s: out of physical memory\n",
						__func__);
				return -1;
			}
			return map_address(
				fault_addr & ~PAGE_MASK,
				1,
				(u32)phy_addr,
				kernel_space);
		}
		record = record->next;
	}
	return -1;
}
