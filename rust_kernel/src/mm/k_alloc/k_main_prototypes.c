
#include "main_headers.h"

#define STDOUT_FILENO 1
#define SIZE_MAX 4294967295

//pthread_mutex_t g_mut = PTHREAD_MUTEX_INITIALIZER;

void			*kmalloc(size_t size)
{
	void		*addr;

//	pthread_mutex_lock(&g_mut);
	if (ctx.is_initialized == false && constructor_runtime() == -1)
		return (NULL);
	if (ctx.tracer_file_descriptor != -1)
		begin_trace(KMALLOC, NULL, size, 0);
	addr = core_allocator(&size);
	if (ctx.tracer_file_descriptor != -1)
		bend_trace(addr != NULL ? SUCCESS : FAIL, addr);
//	pthread_mutex_unlock(&g_mut);
	return (addr);
}

int			kfree(void *ptr)
{
	int ret;

//	pthread_mutex_lock(&g_mut);
	if (ctx.is_initialized == false && constructor_runtime() == -1)
		return -1;
	if (ctx.tracer_file_descriptor != -1)
		begin_trace(KFREE, ptr, 0, 0);
	if (ptr == NULL) {
		if (ctx.tracer_file_descriptor != -1)
			bend_trace(NO_OP, NULL);
//		pthread_mutex_unlock(&g_mut);
		return -1;
	}
	ret = core_deallocator(ptr);
	if (ctx.tracer_file_descriptor != -1)
		bend_trace(ret < 0 ? FAIL : SUCCESS, NULL);
//	pthread_mutex_unlock(&g_mut);
	return 0;
}

size_t			ksize(void *ptr)
{
	size_t size;
//	pthread_mutex_lock(&g_mut);
	if (ctx.is_initialized == false && constructor_runtime() == -1)
		return 0;
	if (ctx.tracer_file_descriptor != -1)
		begin_trace(KSIZE, ptr, 0, 0);
	if (ptr == NULL) {
		if (ctx.tracer_file_descriptor != -1)
			bend_trace(NO_OP, NULL);
//		pthread_mutex_unlock(&g_mut);
		return 0;
	}
	size = get_sizeof_object(ptr);
	if (ctx.tracer_file_descriptor != -1)
		bend_trace(size == 0 ? FAIL : SUCCESS, NULL);
//	pthread_mutex_unlock(&g_mut);
	return size;
}

void			*kcalloc(size_t count, size_t size)
{
	void		*addr;
	size_t		global_size;

//	pthread_mutex_lock(&g_mut);
	if (ctx.is_initialized == false && constructor_runtime() == -1)
		return (NULL);
	if (ctx.tracer_file_descriptor != -1)
		begin_trace(KCALLOC, NULL, count, size);
	global_size = count * size;
	addr = core_allocator(&global_size);
	if (addr != NULL)
		aligned_bzero(addr, global_size);
	if (ctx.tracer_file_descriptor != -1)
		bend_trace(addr != NULL ? SUCCESS : FAIL, addr);
//	pthread_mutex_unlock(&g_mut);
	return (addr);
}

void			*krealloc(void *ptr, size_t size)
{
	void *addr;
	bool memfail;

//	pthread_mutex_lock(&g_mut);
	if (ctx.is_initialized == false && constructor_runtime() == -1)
		return (NULL);
	if (ctx.tracer_file_descriptor != -1)
		begin_trace(KREALLOC, ptr, size, 0);
	if (ptr == NULL) {
		addr = core_allocator(&size);
		if (ctx.tracer_file_descriptor != -1)
			bend_trace(addr != NULL ? SUCCESS : FAIL, addr);
	} else {
		addr = core_realloc(ptr, &size, &memfail);
		if (ctx.tracer_file_descriptor != -1)
			bend_trace(memfail == false ?
					SUCCESS : FAIL, addr);
	}
//	pthread_mutex_unlock(&g_mut);
	return (addr);
}

void			kshow_alloc_mem(void)
{
//	pthread_mutex_lock(&g_mut);
	if (ctx.is_initialized == false && constructor_runtime() == -1)
		return ;
	show_alloc(false, STDOUT_FILENO);
//	pthread_mutex_unlock(&g_mut);
}

void			kshow_alloc_mem_ex(void)
{
//	pthread_mutex_lock(&g_mut);
	if (ctx.is_initialized == false && constructor_runtime() == -1)
		return ;
	show_alloc(true, STDOUT_FILENO);
//	pthread_mutex_unlock(&g_mut);
}
