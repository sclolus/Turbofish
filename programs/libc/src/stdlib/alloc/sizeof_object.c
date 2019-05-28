
#include "main_headers.h"

size_t	get_sizeof_object(void *addr)
{
	struct s_node	*record;
	struct s_node	*index;

	record = alloc_btree_get_node_by_content(
			ctx.big_page_record_tree,
			addr, &cmp_addr_to_node_addr);
	if (record != NULL)
		return record->m.size;

	index = alloc_btree_get_node_by_content(
			ctx.index_pages_tree,
			addr,
			&cmp_addr_to_node_m_addr_range);
	if (index != NULL) {
		record = alloc_btree_get_node_by_content(
				index->ptr_a,
				addr,
				&cmp_addr_to_node_addr);
		if (record != NULL)
			return record->m.size;
	}
	return 0;
}
