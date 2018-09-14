
#include "main_headers.h"

//extern pthread_mutex_t g_mut;

# define GETPAGESIZE() 4096

int	fill_preallocated_chunk(char *base_addr)
{
	ctx.node_pages_entry = (struct s_node_page *)base_addr;
	ctx.node_pages_entry->primary_block.nb_node = 0;
	ctx.node_pages_entry->primary_block.next = NULL;
	ctx.node_cache = NULL;
	base_addr += NODE_REQ_PAGES * ctx.page_size;
	ctx.big_page_record_tree = alloc_btree_new();
	ctx.index_pages_tree = alloc_btree_new();
	ctx.node_density = (NODE_REQ_PAGES * ctx.page_size -
			sizeof(struct s_primary_node)) /
					alloc_btree_get_node_size();
	ctx.global_tiny_space_tree = alloc_btree_new();
	create_index(base_addr, TINY_RANGE);
	insert_free_record(base_addr, TINY_RANGE, TINY, NULL);
	base_addr += TINY_RANGE;
	ctx.global_medium_space_tree = alloc_btree_new();
	create_index(base_addr, MEDIUM_RANGE);
	insert_free_record(base_addr, MEDIUM_RANGE, MEDIUM, NULL);
	return (0);
}

int	constructor_runtime(void)
{
	size_t	preallocated_size;
	void	*base_addr;

	ctx.page_size = GETPAGESIZE();
//	if (getrlimit(RLIMIT_DATA, &ctx.mem_limit) < 0)
	if (0) {
		eprintk("dyn_allocator cannot get RLIMIT_DATA\n");
		return (-1);
	}
	preallocated_size = NODE_REQ_PAGES * ctx.page_size;
	preallocated_size += TINY_RANGE;
	preallocated_size += MEDIUM_RANGE;
	if ((base_addr = get_new_pages(preallocated_size)) == NULL) {
		eprintk("failed to allocate base preallocated memory\n");
		return (-1);
	}
	open_malloc_tracer();
	ctx.size_owned_by_data = 0;
	ctx.size_owned_by_nodes = 0;
	fill_preallocated_chunk(base_addr);
	ctx.is_initialized = true;
	return (0);
}

void __attribute__((constructor))
	main_constructor(void)
{
//	pthread_mutex_lock(&g_mut);
	if (ctx.is_initialized == false)
		constructor_runtime();
//	pthread_mutex_unlock(&g_mut);
}

void __attribute__((destructor))
	main_destructor(void)
{
//	pthread_mutex_lock(&g_mut);
	if (ctx.is_initialized == true)
		close_malloc_tracer();
//	pthread_mutex_unlock(&g_mut);
}
