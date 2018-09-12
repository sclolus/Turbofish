
#include "memory_manager.h"

void			*valloc(size_t size)
{
	void *ptr;

	if ((ptr = vmmap(size)) == (void *)MAP_FAILED)
		return NULL;
	return ptr;
}

int			vfree(void *ptr)
{
	return vmunmap(ptr);
}

size_t			vsize(void *ptr)
{
	(void)ptr;
	return 0;
}
