/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main_prototypes.c                                  :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2018/04/22 17:47:28 by bmickael          #+#    #+#             */
/*   Updated: 2018/04/22 18:14:49 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

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
		begin_trace(MALLOC, NULL, size, 0);
	addr = core_allocator(&size);
	if (ctx.tracer_file_descriptor != -1)
		bend_trace(addr != NULL ? SUCCESS : FAIL, addr);
//	pthread_mutex_unlock(&g_mut);
	return (addr);
}

void			*kcalloc(size_t count, size_t size)
{
	void		*addr;
	size_t		global_size;

//	pthread_mutex_lock(&g_mut);
	if (ctx.is_initialized == false && constructor_runtime() == -1)
		return (NULL);
	if (ctx.tracer_file_descriptor != -1)
		begin_trace(CALLOC, NULL, count, size);
	global_size = count * size;
	addr = core_allocator(&global_size);
	if (addr != NULL)
		ft_aligned_bzero(addr, global_size);
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
		begin_trace(FREE, ptr, 0, 0);
	if (ptr == NULL)
	{
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

void			*krealloc(void *ptr, size_t size)
{
	void *addr;
	bool memfail;

//	pthread_mutex_lock(&g_mut);
	if (ctx.is_initialized == false && constructor_runtime() == -1)
		return (NULL);
	if (ctx.tracer_file_descriptor != -1)
		begin_trace(REALLOC, ptr, size, 0);
	if (ptr == NULL)
	{
		addr = core_allocator(&size);
		if (ctx.tracer_file_descriptor != -1)
			bend_trace(addr != NULL ? SUCCESS : FAIL, addr);
	}
	else
	{
		addr = core_realloc(ptr, &size, &memfail);
		if (ctx.tracer_file_descriptor != -1)
			bend_trace(memfail == false ?
					SUCCESS : FAIL, addr);
	}
//	pthread_mutex_unlock(&g_mut);
	return (addr);
}

void			*kreallocf(void *ptr, size_t size)
{
	void *addr;
	bool memfail;

//	pthread_mutex_lock(&g_mut);
	if (ctx.is_initialized == false && constructor_runtime() == -1)
		return (NULL);
	if (ctx.tracer_file_descriptor != -1)
		begin_trace(REALLOCF, ptr, size, 0);
	memfail = false;
	if (ptr == NULL)
	{
		addr = core_allocator(&size);
		if (addr == NULL)
			memfail = true;
	}
	else
		addr = core_realloc(ptr, &size, &memfail);
	if (memfail == true)
		core_deallocator(ptr);
	if (ctx.tracer_file_descriptor != -1)
		bend_trace(memfail == false ? SUCCESS : FAIL, addr);
//	pthread_mutex_unlock(&g_mut);
	return (addr);
}

static void		*kreallocarray_next(
			void *ptr,
			size_t nmemb,
			size_t size)
{
	size_t		global_size;
	void		*addr;
	bool		memfail;

	global_size = nmemb * size;
	if (ptr == NULL)
	{
		addr = core_allocator(&global_size);
		if (ctx.tracer_file_descriptor != -1)
			bend_trace(addr != NULL ? SUCCESS : FAIL, addr);
	}
	else
	{
		addr = core_realloc(ptr, &global_size, &memfail);
		if (ctx.tracer_file_descriptor != -1)
			bend_trace(memfail == false ? SUCCESS : FAIL, addr);
	}
	return (addr);
}

void			*kreallocarray(void *ptr, size_t nmemb, size_t size)
{
	void				*addr;

//	pthread_mutex_lock(&g_mut);
	if (ctx.is_initialized == false && constructor_runtime() == -1)
		return (NULL);
	if (ctx.tracer_file_descriptor != -1)
		begin_trace(REALLOCARRAY, ptr, nmemb, size);


//	if (nmemb > 0 && (SIZE_MAX / nmemb) < size)
	if (0)
	{
		if (ctx.tracer_file_descriptor != -1)
			bend_trace(FAIL, NULL);
//		errno = ENOMEM;
//		pthread_mutex_unlock(&g_mut);
		return (NULL);
	}
	addr = kreallocarray_next(ptr, nmemb, size);
//	pthread_mutex_unlock(&g_mut);
	return (addr);
}

void			*kvalloc(size_t size)
{
	void		*addr;

//	pthread_mutex_lock(&g_mut);
	if (ctx.is_initialized == false && constructor_runtime() == -1)
		return (NULL);
	if (ctx.tracer_file_descriptor != -1)
		begin_trace(VALLOC, NULL, size, 0);
	if (size == 0)
	{
		if (ctx.tracer_file_descriptor != -1)
			bend_trace(NO_OP, NULL);
//		pthread_mutex_unlock(&g_mut);
		return (NULL);
	}
	size = allign_size(size, LARGE);
	if ((addr = core_allocator_large(&size)) == NULL)
	{
		if (ctx.tracer_file_descriptor != -1)
			bend_trace(FAIL, NULL);
//		errno = ENOMEM;
	}
	else if (ctx.tracer_file_descriptor != -1)
		bend_trace(SUCCESS, addr);
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
