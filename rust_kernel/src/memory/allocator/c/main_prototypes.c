
#include "main_headers.h"

#define STDOUT_FILENO 1
#define SIZE_MAX 4294967295

void *kmalloc(size_t size)
{
	void		*addr;

	if (ctx.is_initialized == false && constructor_runtime() == -1)
		return (NULL);
	addr = core_allocator(&size);
	return (addr);
}

int kfree(void *ptr)
{
	int ret;

	if (ctx.is_initialized == false && constructor_runtime() == -1)
		return -1;
	if (ptr == NULL) {
		return -1;
	}
	ret = core_deallocator(ptr);
	(void)ret;
	return 0;
}

size_t ksize(void *ptr)
{
	size_t size;
	if (ctx.is_initialized == false && constructor_runtime() == -1)
		return 0;
	if (ptr == NULL) {
		return 0;
	}
	size = get_sizeof_object(ptr);
	return size;
}

void *kcalloc(size_t count, size_t size)
{
	void		*addr;
	size_t		global_size;

	if (ctx.is_initialized == false && constructor_runtime() == -1)
		return (NULL);
	global_size = count * size;
	addr = core_allocator(&global_size);
	if (addr != NULL)
		aligned_bzero(addr, global_size);
	return (addr);
}

void *krealloc(void *ptr, size_t size)
{
	void *addr;
	bool memfail;

	if (ctx.is_initialized == false && constructor_runtime() == -1)
		return (NULL);
	if (ptr == NULL) {
		addr = core_allocator(&size);
	} else {
		addr = core_realloc(ptr, &size, &memfail);
	}
	return (addr);
}
