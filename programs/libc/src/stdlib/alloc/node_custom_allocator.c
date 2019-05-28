
#include "main_headers.h"

static struct s_node_page	*assign_new_node_page(void)
{
	struct s_node_page *new_node_page;

	if (ctx.node_cache == NULL) {
		new_node_page = (struct s_node_page *)
				get_new_pages(NODE_REQ_PAGES * ctx.page_size);
	} else {
		new_node_page = ctx.node_cache;
		ctx.node_cache = NULL;
	}
	return (new_node_page);
}

void				*node_custom_allocator(size_t size)
{
	struct s_node_page	*node_page;
	struct s_node_page	*new_node_page;
	void			*addr;

	node_page = ctx.node_pages_entry;
	if (node_page->primary_block.nb_node == ctx.node_density) {
		if ((new_node_page = assign_new_node_page()) == NULL)
			return (NULL);
		new_node_page->primary_block.nb_node = 0;
		new_node_page->primary_block.next = node_page;
		ctx.node_pages_entry = new_node_page;
		node_page = new_node_page;
	}
	addr = &node_page->node[node_page->primary_block.nb_node];
	node_page->primary_block.nb_node += 1;
	ctx.size_owned_by_nodes += sizeof(struct s_node);
	(void)size;
	return (addr);
}
