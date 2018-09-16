
#include "main_headers.h"

//extern pthread_mutex_t g_mut;

# define GETPAGESIZE() 4096

int	fill_preallocated_chunk(void)
{
	void *medium_base_addr;
	void *tiny_base_addr;
	void *metanode_base_addr;

	medium_base_addr = get_new_pages(MEDIUM_RANGE);
	if (medium_base_addr == NULL) {
		eprintk("failed to allocate base medium page memory\n");
		return -1;
	}

	tiny_base_addr = get_new_pages(TINY_RANGE);
	if (tiny_base_addr == NULL) {
		eprintk("failed to allocate base tiny page memory\n");
		return -1;
	}

	metanode_base_addr = get_new_pages(NODE_REQ_PAGES * ctx.page_size);
	if (metanode_base_addr == NULL) {
		eprintk("failed to allocate base meta node memory\n");
		return -1;
	}

	ctx.node_pages_entry = (struct s_node_page *)metanode_base_addr;

	ctx.node_pages_entry->primary_block.nb_node = 0;
	ctx.node_pages_entry->primary_block.next = NULL;
	ctx.node_cache = NULL;
	ctx.big_page_record_tree = alloc_btree_new();
	ctx.index_pages_tree = alloc_btree_new();
	ctx.node_density = (NODE_REQ_PAGES * ctx.page_size -
			sizeof(struct s_primary_node)) /
					alloc_btree_get_node_size();

	ctx.global_tiny_space_tree = alloc_btree_new();
	create_index(tiny_base_addr, TINY_RANGE);
	insert_free_record(tiny_base_addr, TINY_RANGE, TINY, NULL);

	ctx.global_medium_space_tree = alloc_btree_new();
	create_index(medium_base_addr, MEDIUM_RANGE);
	insert_free_record(medium_base_addr, MEDIUM_RANGE, MEDIUM, NULL);
	return (0);
}

int	constructor_runtime(void)
{
	int ret;

	ctx.page_size = GETPAGESIZE();
	open_malloc_tracer();
	ctx.size_owned_by_data = 0;
	ctx.size_owned_by_nodes = 0;
	ret = fill_preallocated_chunk();
	ctx.is_initialized = (ret == 0) ? true : false;
	return ret;
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
