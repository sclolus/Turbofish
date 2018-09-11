
#include "memory_manager.h"

void			*valloc(size_t size)
{
	(void)size;
	return NULL;
}

void			vfree(void *ptr)
{
	(void)ptr;
}

size_t			vsize(void *ptr)
{
	(void)ptr;
	return 0;
}
