
#include "alloc_btree_internal_header.h"

void		alloc_btree_swap_data(
		struct s_node *node_a,
		struct s_node *node_b)
{
	void	*content;
	void	*size;
	uint8_t	node_type;

	content = node_a->ptr_a;
	node_a->ptr_a = node_b->ptr_a;
	node_b->ptr_a = content;
	size = node_a->m.ptr_b;
	node_a->m.ptr_b = node_b->m.ptr_b;
	node_b->m.ptr_b = size;
	node_type = node_a->mask.s.node_type;
	node_a->mask.s.node_type = node_b->mask.s.node_type;
	node_b->mask.s.node_type = node_type;
	node_type = node_a->mask.s.range;
	node_a->mask.s.range = node_b->mask.s.range;
	node_b->mask.s.range = node_type;
}
