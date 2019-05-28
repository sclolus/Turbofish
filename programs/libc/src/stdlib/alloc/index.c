
#include "main_headers.h"

void	*insert_allocated_record(struct s_node *record)
{
	struct s_node *index;

	index = (struct s_node *)alloc_btree_get_node_by_content(
			ctx.index_pages_tree,
			record->ptr_a,
			&cmp_addr_to_node_m_addr_range);
	record = alloc_btree_insert_rnb_node((struct s_node **)&index->ptr_a,
			record, cmp_node_addr_to_node_addr);
	if (record == NULL)
		return (NULL);
	ctx.size_owned_by_data += record->m.size;
	return (record->ptr_a);
}

void	*create_index(
	void *addr,
	uint32_t range)
{
	struct s_node *index;

	index = alloc_btree_create_node(&node_custom_allocator);
	if (index == NULL)
		return (NULL);
	index->ptr_a = alloc_btree_new();
	index->m.ptr_b = addr;
	index->mask.s.range = range;
	index->mask.s.node_type = INDEX;
	index = alloc_btree_insert_rnb_node(
			&ctx.index_pages_tree,
			index,
			&cmp_node_m_addr_to_node_m_addr);
	return (index);
}

/*
** When a index if destroyed. Allocated pages block return to system.
*/

void	destroy_index(struct s_node *index)
{
	alloc_btree_delete_rnb_node(
			&ctx.index_pages_tree,
			index,
			&node_custom_deallocator);
}
